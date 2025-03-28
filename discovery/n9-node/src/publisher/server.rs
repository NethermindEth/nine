use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next};
use crb::superagent::{EventBridge, StreamSession, Supervisor, SupervisorSession};
use std::sync::LazyLock;

static PUB_BRIDGE: LazyLock<EventBridge<Delegate>> = LazyLock::new(|| EventBridge::new());

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
        // ctx.consume(PUB_BRIDGE.events().await?);
        Ok(Next::events())
    }
}

pub struct Delegate {}
