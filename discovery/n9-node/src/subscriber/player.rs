use super::{Projection, StateEvent};
use crate::atom::{PackedDelta, State};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::core::{mpsc, watch};
use crb::superagent::{OnRequest, Request};
use std::marker::PhantomData;

pub struct Player<S: State> {
    state_tx: Option<watch::Sender<S>>,
    event_tx: mpsc::UnboundedSender<StateEvent<S>>,
    event_rx: Option<mpsc::UnboundedReceiver<StateEvent<S>>>,
}

impl<S: State> Player<S> {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            state_tx: None,
            event_tx: tx,
            event_rx: Some(rx),
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
    type Response = mpsc::UnboundedReceiver<StateEvent<S>>;
}

#[async_trait]
impl<S: State> OnRequest<GetDeltasChannel<S>> for Player<S> {
    async fn on_request(
        &mut self,
        _: GetDeltasChannel<S>,
        _ctx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<StateEvent<S>>> {
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
        let packed_query = S::pack_query(&event.query)?;
        // TODO: Forward to a recorder
        Ok(())
    }
}

pub struct ProcessDelta {
    delta: PackedDelta,
}

#[async_trait]
impl<S: State> OnEvent<ProcessDelta> for Player<S> {
    async fn handle(&mut self, event: ProcessDelta, _ctx: &mut Context<Self>) -> Result<()> {
        let delta = S::unpack_delta(&event.delta)?;
        self.apply_delta(delta)?;
        Ok(())
    }
}

impl<S: State> Player<S> {
    fn allocate_state(&mut self, new_state: S) -> Result<()> {
        let (state, state_tx) = Projection::new(new_state);
        self.state_tx = Some(state_tx);
        let event = StateEvent::State(state);
        self.send(event)
    }

    fn deallocate_state(&mut self) -> Result<()> {
        self.state_tx.take();
        self.send(StateEvent::Lost)
    }

    fn apply_delta(&mut self, delta: S::Delta) -> Result<()> {
        let state_tx = self
            .state_tx
            .as_mut()
            .ok_or_else(|| anyhow!("No state to apply a delta"))?;
        state_tx.send_modify(|state| {
            state.apply(delta.clone());
        });
        self.send(StateEvent::Delta(delta))?;
        Ok(())
    }

    fn send(&self, event: StateEvent<S>) -> Result<()> {
        if self.event_tx.is_closed() {
            Err(anyhow!("State deltas channel is closed"))
        } else {
            self.event_tx.send(event)?;
            Ok(())
        }
    }
}
