use super::StateEvent;
use crate::atom::State;
use crate::publisher::Query;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
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

pub struct Deltas<S> {
    _type: PhantomData<S>,
}

impl<S: State> Deltas<S> {
    pub fn new() -> Self {
        Self { _type: PhantomData }
    }
}

impl<S: State> Request for Deltas<S> {
    type Response = mpsc::UnboundedReceiver<StateEvent<S>>;
}

#[async_trait]
impl<S: State> OnRequest<Deltas<S>> for Player<S> {
    async fn on_request(
        &mut self,
        _: Deltas<S>,
        _ctx: &mut Context<Self>,
    ) -> Result<mpsc::UnboundedReceiver<StateEvent<S>>> {
        self.event_rx
            .take()
            .ok_or_else(|| anyhow!("A deltas receiver has taken already."))
    }
}
