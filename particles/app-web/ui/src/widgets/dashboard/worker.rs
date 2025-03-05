use super::flow::Dashboard;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::StreamSession;
use ui9_dui::{Act, Pub};

pub struct DashboardWorker {
    state: Pub<Dashboard>,
}

impl DashboardWorker {
    pub fn new() -> Self {
        Self {
            state: Pub::unified(),
        }
    }
}

impl Agent for DashboardWorker {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for DashboardWorker {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        ctx.consume(self.state.actions()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<Dashboard>> for DashboardWorker {
    async fn handle(&mut self, msg: Act<Dashboard>, ctx: &mut Context<Self>) -> Result<()> {
        self.state.event(msg.action);
        Ok(())
    }
}
