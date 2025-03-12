use crate::flow::chat_control::{ChatControl, ChatControlAction, Role};
use crate::flow::session_control::SessionKey;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::superagent::{Fetcher, StreamSession, Supervisor};
use n9_core::{ChatRequest, ChatResponse, RouterLink};
use ui9_dui::{Act, Operation, Pub};

pub struct ChatControlLoop {
    router: RouterLink,
    chat: Pub<ChatControl>,
}

impl ChatControlLoop {
    pub fn new(router: RouterLink, key: SessionKey) -> Self {
        Self {
            router,
            chat: Pub::new(key),
        }
    }
}

impl Agent for ChatControlLoop {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for ChatControlLoop {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        ctx.consume(self.chat.actions()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<ChatControl>> for ChatControlLoop {
    async fn handle(&mut self, msg: Act<ChatControl>, ctx: &mut Context<Self>) -> Result<()> {
        match msg.action {
            ChatControlAction::Prompt { prompt } => {
                let ask = SendRequest { prompt };
                ctx.do_next(Next::do_async(ask));
            }
        }
        Ok(())
    }
}

struct SendRequest {
    prompt: String,
}

#[async_trait]
impl DoAsync<SendRequest> for ChatControlLoop {
    async fn handle(&mut self, msg: SendRequest, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let op = Operation::start("Sending a prompt");
        self.chat.thinking(true);
        let request = ChatRequest::user(&msg.prompt);
        let session = self.router.new_session().await?;
        let req = session.chat(request);
        self.chat.add(msg.prompt, Role::Request);
        op.end();
        let state = WaitResponse { req };
        Ok(Next::do_async(state))
    }

    async fn fallback(&mut self, err: Error) -> Next<Self> {
        self.chat.thinking(false);
        // TODO: Operation failure reporting here
        Next::events()
    }
}

struct WaitResponse {
    req: Fetcher<ChatResponse>,
}

#[async_trait]
impl DoAsync<WaitResponse> for ChatControlLoop {
    async fn handle(&mut self, msg: WaitResponse, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let op = Operation::start("Waiting for the response");
        let resp = msg.req.await?.squash();
        self.chat.add(resp, Role::Response);
        self.chat.thinking(false);
        op.end();
        Ok(Next::events())
    }

    async fn fallback(&mut self, err: Error) -> Next<Self> {
        self.chat.thinking(false);
        // TODO: Operation failure reporting here
        Next::events()
    }
}
