use crate::flow::chat_control::{ChatControl, ChatControlAction, Role};
use crate::flow::session_control::SessionKey;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::core::uuid::Uuid;
use crb::superagent::{Fetcher, StreamSession, Supervisor};
use n9_core::chain::ReasoningFlow;
use n9_core::{ChatRequest, ChatResponse, RouterLink};
use ui9::names::Fqn;
use ui9_dui::{Act, Operation, Pub};
use ui9_net::{FqnLink, MeshNode};

pub struct ChatControlLoop {
    key: SessionKey,
    router: RouterLink,
    chat: Pub<ChatControl>,
    tracer: Option<Pub<ReasoningFlow>>,
}

impl ChatControlLoop {
    pub fn new(router: RouterLink, key: SessionKey) -> Self {
        Self {
            key: key.clone(),
            router,
            chat: Pub::new(key),
            tracer: None,
        }
    }
}

impl Agent for ChatControlLoop {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

impl ChatControlLoop {
    pub fn create_tracer(&mut self) -> Result<FqnLink> {
        let uuid = Uuid::new_v4();
        let fqn = self.key.push(uuid);
        let tracer = Pub::new(fqn.clone());
        self.tracer = Some(tracer);
        let peer_id = MeshNode::link()?.peer_id;
        Ok(FqnLink::remote(fqn, peer_id))
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
        let tracer = self.create_tracer()?;

        // let op = Operation::start("Sending a prompt");
        self.chat.start_thinking("Sending a prompt");

        let request = ChatRequest::user(&msg.prompt);
        let session = self.router.new_session().await?;
        // TODO: Assign a tracer here
        let req = session.chat(request);
        self.chat.add(msg.prompt, Role::Request);

        // op.end();
        let state = WaitResponse { req };
        Ok(Next::do_async(state))
    }

    async fn fallback(&mut self, err: Error) -> Next<Self> {
        self.chat.stop_thinking();
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
        // TODO: Use chat scoped operations
        // let op = Operation::start("Waiting for a response");
        self.chat.start_thinking("Waiting for a response");

        let resp = msg.req.await?.squash();
        self.chat.add(resp, Role::Response);

        // op.end();
        self.chat.stop_thinking();
        Ok(Next::events())
    }

    async fn fallback(&mut self, err: Error) -> Next<Self> {
        self.chat.stop_thinking();
        // TODO: Operation failure reporting here
        Next::events()
    }
}
