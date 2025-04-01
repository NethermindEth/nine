use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Next, DoAsync};
use crb::superagent::StreamSession;

pub struct ConfigToml {
}

impl ConfigToml {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Agent for ConfigToml {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for ConfigToml {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        Ok(Next::events())
    }
}
