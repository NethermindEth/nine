use anyhow::Result;
use async_trait::async_trait;
use config_toml::{ConfigState, LiveConfig};
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::superagent::StreamSession;
use n9_node::{AtomId, Projection, Sub, SubEvent};

pub struct ModelOpenAI {
    config_sub: Sub<ConfigState<()>>,
    config: Option<LiveConfig<()>>,
}

impl Agent for ModelOpenAI {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

impl ModelOpenAI {
    pub fn new() -> Self {
        let id = AtomId::local("@n9-config-toml");
        Self {
            config_sub: Sub::connect(id),
            config: None,
        }
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for ModelOpenAI {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let config_events = self.config_sub.events().await?;
        ctx.consume(config_events);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<SubEvent<ConfigState<()>>> for ModelOpenAI {
    async fn handle(
        &mut self,
        event: SubEvent<ConfigState<()>>,
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        match event {
            SubEvent::State(state) => {
                self.config = Some(state);
            }
            SubEvent::Delta(_) => {}
            SubEvent::Lost => {}
        }
        Ok(())
    }
}
