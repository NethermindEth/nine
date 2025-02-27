use crate::tools::TaskInfo;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Next, DoAsync, OnEvent};
use crb::superagent::{Timer, StreamSession, Timeout};

pub struct ChatTask {
    task_info: TaskInfo,
    timer: Timer,
}

impl ChatTask {
    pub fn new(task_info: TaskInfo) -> Self {
        Self {
            task_info,
            timer: Timer::new(),
        }
    }
}

impl Agent for ChatTask {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for ChatTask {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        ctx.consume(self.timer.events()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Timeout> for ChatTask {
    async fn handle(&mut self, _: Timeout, _ctx: &mut Context<Self>) -> Result<()> {
        if self.task_info.repeat {
        }
        Ok(())
    }
}
