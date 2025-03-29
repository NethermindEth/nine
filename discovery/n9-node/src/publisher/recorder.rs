use super::Query;
use crate::atom::State;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, OnEvent};
use crb::core::mpsc;
use crb::superagent::{OnRequest, Request};
use std::marker::PhantomData;

pub struct Recorder<S: State> {
    state: S,
    query_tx: mpsc::UnboundedSender<Query<S>>,
    query_rx: Option<mpsc::UnboundedReceiver<Query<S>>>,
}

impl<S: State> Agent for Recorder<S> {
    type Context = AgentSession<Self>;
}

impl<S: State> Recorder<S> {
    pub fn new(state: S) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            state,
            query_tx: tx,
            query_rx: Some(rx),
        }
    }
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

pub struct Delta<S: State> {
    delta: S::Delta,
}

impl<S: State> Delta<S> {
    pub fn new(delta: S::Delta) -> Self {
        Self { delta }
    }
}

#[async_trait]
impl<S: State> OnEvent<Delta<S>> for Recorder<S> {
    async fn handle(&mut self, event: Delta<S>, _ctx: &mut Context<Self>) -> Result<()> {
        // self.distribute(update.event)?;
        Ok(())
    }
}
