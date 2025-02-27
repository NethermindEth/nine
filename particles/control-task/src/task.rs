use crate::tools::TaskInfo;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Next, DoAsync, OnEvent, ManagedContext};
use crb::superagent::{Timer, StreamSession, Timeout};
use crb::core::time::Duration;
use ui9_dui::Sub;
use n9_control_chat::Chat;

pub struct ChatTask {
    task_info: TaskInfo,
    timer: Timer,
    chat: Sub<Chat>,
}

impl ChatTask {
    pub fn new(task_info: TaskInfo) -> Self {
        Self {
            task_info,
            timer: Timer::new(),
            chat: Sub::local_unified(),
        }
    }
}

impl Agent for ChatTask {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

impl ChatTask {
    fn schedule(&mut self) -> Result<()> {
        let duration = Duration::from_secs(self.task_info.interval_sec);
        self.timer.schedule(duration)?;
        Ok(())
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for ChatTask {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        ctx.consume(self.timer.events()?);
        self.schedule()?;
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Timeout> for ChatTask {
    async fn handle(&mut self, _: Timeout, ctx: &mut Context<Self>) -> Result<()> {
        let prompt = self.task_info.prompt.clone();
        self.chat.request(prompt);

        if self.task_info.repeat {
            self.schedule()?;
        } else {
            ctx.shutdown();
        }
        Ok(())
    }
}
