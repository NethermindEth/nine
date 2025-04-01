use super::player::Player;
use crate::subscriber::State;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, OnEvent, RunAgent};
use crb::runtime::{InteractiveRuntime, Runtime};
use crb::superagent::{EventBridge, StreamSession, Supervisor, SupervisorSession};
use std::sync::LazyLock;

static PUB_BRIDGE: LazyLock<EventBridge<Delegate>> = LazyLock::new(|| EventBridge::new());

impl HubClient {
    pub fn spawn_player<S>(player: Player<S>) -> Address<Player<S>>
    where
        S: State,
    {
        let runtime = RunAgent::new(player);
        let address = runtime.address();
        let delegate = Delegate {
            runtime: Box::new(runtime),
        };
        PUB_BRIDGE.send(delegate);
        address
    }
}

pub struct HubClient {}

impl HubClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl Supervisor for HubClient {
    type BasedOn = StreamSession<Self>;
    type GroupBy = ();
}

impl Agent for HubClient {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for HubClient {
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
impl OnEvent<Delegate> for HubClient {
    async fn handle(&mut self, delegate: Delegate, ctx: &mut Context<Self>) -> Result<()> {
        ctx.spawn_trackable(delegate.runtime, ());
        Ok(())
    }
}
