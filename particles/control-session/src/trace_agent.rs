use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::StreamSession;
use n9_core::chain::ReasoningFlow;
use ui9::names::Fqn;
use ui9_dui::{Act, Pub};

pub struct TraceAgent {
    tracer: Pub<ReasoningFlow>,
}

impl TraceAgent {
    pub fn new(fqn: Fqn) -> Self {
        Self {
            tracer: Pub::new(fqn),
        }
    }
}

impl Agent for TraceAgent {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for TraceAgent {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        ctx.consume(self.tracer.actions()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<ReasoningFlow>> for TraceAgent {
    async fn handle(&mut self, msg: Act<ReasoningFlow>, ctx: &mut Context<Self>) -> Result<()> {
        self.tracer.event(msg.action);
        Ok(())
    }
}
