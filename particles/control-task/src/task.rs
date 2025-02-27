use crate::tools::TaskParameters;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, Next, DoAsync, OnEvent, ManagedContext};
use crb::superagent::{Timer, StreamSession, Timeout};
use crb::core::time::Duration;
use ui9_dui::Sub;
use n9_control_chat::Chat;

pub struct ChatTask {
    parameters: TaskParameters,
    timer: Timer,
    chat: Sub<Chat>,
}

impl ChatTask {
    pub fn new(parameters: TaskParameters) -> Self {
        Self {
            parameters,
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
        let duration = Duration::from_secs(self.parameters.interval_sec);
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
        let prompt = self.parameters.prompt.clone();
        self.chat.request(prompt);

        if self.parameters.repeat {
            self.schedule()?;
        } else {
            ctx.shutdown();
        }
        Ok(())
    }
}
