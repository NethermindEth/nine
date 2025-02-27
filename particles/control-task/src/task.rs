use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, Next, DoAsync};

pub struct ChatTask {
}

impl Agent for ChatTask {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for ChatTask {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        Ok(Next::events())
    }
}
