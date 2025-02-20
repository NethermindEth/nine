use crate::tracers::event::Event;
use crate::tracers::failure::Failure;
use crate::tracers::job::Job;
use crate::{Act, Pub};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::{AgentBridge, StreamSession};
use std::sync::LazyLock;

pub static LOG_BRIDGE: LazyLock<AgentBridge<Reporter>> = LazyLock::new(|| AgentBridge::new());

pub struct Reporter {
    job: Pub<Job>,
    event: Pub<Event>,
    failure: Pub<Failure>,
}

impl Reporter {
    pub fn new() -> Self {
        Self {
            job: Pub::unified(),
            event: Pub::unified(),
            failure: Pub::unified(),
        }
    }
}

impl Agent for Reporter {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Reporter {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        ctx.consume_events(LOG_BRIDGE.events().await?);
        ctx.consume(self.job.actions()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<Job>> for Reporter {
    async fn handle(&mut self, msg: Act<Job>, _ctx: &mut Context<Self>) -> Result<()> {
        self.job.event(msg.action);
        Ok(())
    }
}

#[async_trait]
impl OnEvent<Act<Event>> for Reporter {
    async fn handle(&mut self, msg: Act<Event>, _ctx: &mut Context<Self>) -> Result<()> {
        self.event.event(msg.action);
        Ok(())
    }
}

#[async_trait]
impl OnEvent<Act<Failure>> for Reporter {
    async fn handle(&mut self, msg: Act<Failure>, _ctx: &mut Context<Self>) -> Result<()> {
        self.failure.event(msg.action);
        Ok(())
    }
}
