use super::{Projection, SubEvent};
use crate::atom::{PackedDelta, State, TypedAtomId};
use crate::node::Node;
use crate::publisher::{DeltaFlow, RecorderLink, StateId};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::core::{mpsc, watch, Slot};
use crb::superagent::{Entry, OnRequest, Request};
use std::marker::PhantomData;

struct Binding {
    state_id: StateId,
    recorder: RecorderLink,
    #[allow(unused)]
    entry: Entry<DeltaFlow>,
}

pub struct Player<S: State> {
    atom: TypedAtomId<S>,
    /// A sender for state events: new_state, delta, lost.
    event_tx: mpsc::UnboundedSender<SubEvent<S>>,
    /// A receiver for state events that is used by a listener.
    event_rx: Option<mpsc::UnboundedReceiver<SubEvent<S>>>,
    binding: Slot<Binding>,
    /// A sender for a projected state, when it's unpacked
    state_tx: Option<watch::Sender<S>>,
}

impl<S: State> Player<S> {
    pub fn new(atom: TypedAtomId<S>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            atom,
            event_tx: tx,
            event_rx: Some(rx),
            binding: Slot::empty(),
            state_tx: None,
        }
    }
}

impl<S: State> Agent for Player<S> {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl<S: State> DoAsync<Initialize> for Player<S> {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let node = Node::link()?;
        if self.atom.same_peer(node.peer) {
            let aqn = self.atom.path.clone();
            let mut recorder = node.server.discover(aqn).await?;
            let recipient = ctx.recipient();
            let state_entry = recorder.subscribe(recipient).await?;

            // Assign the initial state
            let state_init = state_entry.state;
            let unpacked_state = S::unpack_state(&state_init.state)?;
            self.allocate_state(unpacked_state)?;

            // Store subscription handle and a link to forward actions
            let binding = Binding {
                state_id: state_init.state_id,
                recorder,
                entry: state_entry.entry,
            };
            self.binding.fill(binding)?;
        } else {
            return Err(anyhow!("Not implemented: p2p player connections"));
            // Use a connector
        }
        Ok(Next::events())
    }
}

pub struct GetDeltasChannel<S> {
    _type: PhantomData<S>,
}

impl<S: State> GetDeltasChannel<S> {
    pub fn new() -> Self {
        Self { _type: PhantomData }
    }
}

impl<S: State> Request for GetDeltasChannel<S> {
    type Response = mpsc::UnboundedReceiver<SubEvent<S>>;
}

#[async_trait]
impl<S: State> OnRequest<GetDeltasChannel<S>> for Player<S> {
    async fn on_request(
        &mut self,
        _: GetDeltasChannel<S>,
        _ctx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<SubEvent<S>>> {
        self.event_rx
            .take()
            .ok_or_else(|| anyhow!("A deltas receiver has taken already."))
    }
}

pub struct SendQuery<S: State> {
    query: S::Query,
}

impl<S: State> SendQuery<S> {
    pub fn new(query: S::Query) -> Self {
        Self { query }
    }
}

#[async_trait]
impl<S: State> OnEvent<SendQuery<S>> for Player<S> {
    async fn handle(&mut self, event: SendQuery<S>, _ctx: &mut Context<Self>) -> Result<()> {
        let binding = self.binding.get_mut()?;
        let packed_query = S::pack_query(&event.query)?;
        binding
            .recorder
            .query(binding.state_id, packed_query)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<S: State> OnEvent<PackedDelta> for Player<S> {
    async fn handle(&mut self, delta: PackedDelta, _ctx: &mut Context<Self>) -> Result<()> {
        let delta = S::unpack_delta(&delta)?;
        self.apply_delta(delta)?;
        Ok(())
    }
}

impl<S: State> Player<S> {
    fn allocate_state(&mut self, new_state: S) -> Result<()> {
        let (state, state_tx) = Projection::new(new_state);
        self.state_tx = Some(state_tx);
        let event = SubEvent::State(state);
        self.send(event)
    }

    fn deallocate_state(&mut self) -> Result<()> {
        self.state_tx.take();
        self.send(SubEvent::Lost)
    }

    fn apply_delta(&mut self, delta: S::Delta) -> Result<()> {
        let state_tx = self
            .state_tx
            .as_mut()
            .ok_or_else(|| anyhow!("No state to apply a delta"))?;
        state_tx.send_modify(|state| {
            state.apply(delta.clone());
        });
        self.send(SubEvent::Delta(delta))?;
        Ok(())
    }

    fn send(&self, event: SubEvent<S>) -> Result<()> {
        if self.event_tx.is_closed() {
            Err(anyhow!("State deltas channel is closed"))
        } else {
            self.event_tx.send(event)?;
            Ok(())
        }
    }
}
