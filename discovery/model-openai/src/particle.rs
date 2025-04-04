use anyhow::Result;
use async_trait::async_trait;
use config_toml::ConfigState;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use crb::superagent::StreamSession;
use n9_node::Sub;

pub struct ModelOpenAI {
    state: Sub<ConfigState<()>>,
}

impl Agent for ModelOpenAI {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for ModelOpenAI {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        Ok(Next::events())
    }
}
