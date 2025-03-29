use super::recorder::Recorder;
use crate::publisher::State;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, OnEvent, RunAgent};
use crb::runtime::{InteractiveRuntime, Runtime};
use crb::superagent::{EventBridge, StreamSession, Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From};
use std::sync::LazyLock;

static PUB_BRIDGE: LazyLock<EventBridge<Delegate>> = LazyLock::new(|| EventBridge::new());

#[derive(Deref, DerefMut, From, Clone)]
pub struct HubServerLink {
    address: Address<HubServer>,
}

impl HubServer {
    pub fn spawn_recorder<S>(state: S) -> Address<Recorder<S>>
    where
        S: State,
    {
        let recorder = Recorder::new(state);
        let runtime = RunAgent::new(recorder);
        let address = runtime.address();
        let delegate = Delegate {
            runtime: Box::new(runtime),
        };
        PUB_BRIDGE.send(delegate);
        address
    }
}

pub struct HubServer {}

impl HubServer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Supervisor for HubServer {
    type BasedOn = StreamSession<Self>;
    type GroupBy = ();
}

impl Agent for HubServer {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for HubServer {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let events = PUB_BRIDGE.events().await?;
        ctx.consume(events);
        Ok(Next::events())
    }
}

pub struct Delegate {
    runtime: Box<dyn Runtime>,
}

#[async_trait]
impl OnEvent<Delegate> for HubServer {
    async fn handle(&mut self, delegate: Delegate, ctx: &mut Context<Self>) -> Result<()> {
        ctx.spawn_trackable(delegate.runtime, ());
        Ok(())
    }
}
