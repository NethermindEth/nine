use crate::flow::TaskPub;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, ManagedContext, Next, OnEvent};
use crb::superagent::{StreamSession, Timeout, Timer};
use n9_control_chat::Chat;
use ui9::names::Fqn;
use ui9_dui::Sub;

pub struct ChatTask {
    state: TaskPub,
    timer: Timer,
    chat: Sub<Chat>,
}

impl ChatTask {
    pub fn new(state: TaskPub, chat: Fqn) -> Self {
        Self {
            state,
            timer: Timer::new(),
            chat: Sub::local_unified(),
        }
    }
}

impl Agent for ChatTask {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        self.state.register();
        Next::do_async(Initialize)
    }

    fn interrupt(&mut self, ctx: &mut Context<Self>) {
        self.timer.cancel().ok();
        ctx.shutdown();
    }

    fn end(&mut self) {
        self.state.unregister();
    }
}

impl ChatTask {
    fn schedule(&mut self) -> Result<()> {
        let duration = self.state.duration()?;
        self.timer.schedule(duration)?;
        self.state.update();
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
        let prompt = self.state.prompt.clone();
        self.chat.request(prompt);

        if self.state.repeat {
            self.schedule()?;
        } else {
            self.interrupt(ctx);
        }
        Ok(())
    }
}
