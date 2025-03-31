use super::{Query, StateId};
use crate::atom::{PackedDelta, PackedQuery, PackedState, State};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, OnEvent, UniAddress};
use crb::core::{mpsc, Unique};
use crb::send::{Recipient, Sender};
use crb::superagent::{
    Fetcher, InteractExt, ManageSubscription, OnRequest, Request, StateEntry, SubscribeExt,
    Subscription,
};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct RecorderLink {
    address: UniAddress<dyn TypelessRecorder>,
}

impl RecorderLink {
    pub fn new(address: impl TypelessRecorder) -> Self {
        Self {
            address: UniAddress::new(address),
        }
    }

    pub async fn subscribe(
        &mut self,
        recipient: Recipient<PackedDelta>,
    ) -> Result<StateEntry<DeltaFlow>> {
        let state_id = StateId::unique();
        let msg = DeltaFlow {
            state_id,
            recipient,
        };
        let state = self.address.subscribe(msg).await?;
        Ok(state)
    }

    pub fn query(&mut self, from: StateId, query: PackedQuery) -> Fetcher<()> {
        let query = ProcessQuery { from, query };
        self.address.interact(query)
    }
}

pub trait TypelessRecorder
where
    Self: Sync + Send + 'static,
    Self: SubscribeExt<DeltaFlow>,
    Self: InteractExt<ProcessQuery>,
{
}

impl<S: State> TypelessRecorder for Address<Recorder<S>> {}

pub struct Recorder<S: State> {
    state: S,
    query_tx: mpsc::UnboundedSender<Query<S>>,
    query_rx: Option<mpsc::UnboundedReceiver<Query<S>>>,
    subscribers: HashMap<StateId, Unique<DeltaFlow>>,
}

impl<S: State> Recorder<S> {
    pub fn new(state: S) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            state,
            query_tx: tx,
            query_rx: Some(rx),
            subscribers: HashMap::new(),
        }
    }
}

impl<S: State> Agent for Recorder<S> {
    type Context = AgentSession<Self>;
}

pub struct Queries<S> {
    _type: PhantomData<S>,
}

impl<S: State> Queries<S> {
    pub fn new() -> Self {
        Self { _type: PhantomData }
    }
}

impl<S: State> Request for Queries<S> {
    type Response = mpsc::UnboundedReceiver<Query<S>>;
}

#[async_trait]
impl<S: State> OnRequest<Queries<S>> for Recorder<S> {
    async fn on_request(
        &mut self,
        _: Queries<S>,
        _ctx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<Query<S>>> {
        self.query_rx
            .take()
            .ok_or_else(|| anyhow!("A queries receiver has taken already."))
    }
}

pub struct SendDelta<S: State> {
    state_id: Option<StateId>,
    delta: S::Delta,
}

impl<S: State> SendDelta<S> {
    pub fn new(state_id: Option<StateId>, delta: S::Delta) -> Self {
        Self { state_id, delta }
    }
}

#[async_trait]
impl<S: State> OnEvent<SendDelta<S>> for Recorder<S> {
    async fn handle(&mut self, event: SendDelta<S>, _ctx: &mut Context<Self>) -> Result<()> {
        let packed_delta = S::pack_delta(&event.delta)?;
        if let Some(state_id) = event.state_id {
            if let Some(sub) = self.subscribers.get(&state_id) {
                sub.recipient.send(packed_delta)
            } else {
                Err(anyhow!(
                    "No state with id {state_id:?} to send a delta directly"
                ))
            }
        } else {
            self.state.apply(event.delta);
            for (_id, sub) in &self.subscribers {
                // TODO: Collect errors
                sub.recipient.send(packed_delta.clone()).ok();
            }
            Ok(()) // TODO: Multi-result
        }
    }
}

pub struct DeltaFlow {
    state_id: StateId,
    recipient: Recipient<PackedDelta>,
}

impl Subscription for DeltaFlow {
    type State = PackedState;
}

#[async_trait]
impl<S: State> ManageSubscription<DeltaFlow> for Recorder<S> {
    async fn subscribe(
        &mut self,
        sub: Unique<DeltaFlow>,
        _ctx: &mut Context<Self>,
    ) -> Result<PackedState> {
        let state_id = sub.state_id.clone();
        let state = self.state.divide();
        let packed_state = state.pack_state()?;
        self.subscribers.insert(state_id, sub);
        Ok(packed_state)
    }

    async fn unsubscribe(
        &mut self,
        sub: Unique<DeltaFlow>,
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        self.subscribers.remove(&sub.state_id);
        Ok(())
    }
}

pub struct ProcessQuery {
    from: StateId,
    query: PackedQuery,
}

impl Request for ProcessQuery {
    type Response = ();
}

#[async_trait]
impl<S: State> OnRequest<ProcessQuery> for Recorder<S> {
    async fn on_request(&mut self, request: ProcessQuery, _ctx: &mut Context<Self>) -> Result<()> {
        let ProcessQuery { from, query } = request;
        let query = S::unpack_query(&query)?;
        let msg = Query { from, query };
        self.query_tx.send(msg)?;
        Ok(())
    }
}
