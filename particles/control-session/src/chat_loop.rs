use crate::flow::chat_control::{ChatControl, ChatControlAction, Role};
use crate::flow::session_control::SessionKey;
use crate::trace_agent::TraceAgent;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Equip, Next, OnEvent};
use crb::core::uuid::Uuid;
use crb::superagent::{Fetcher, PingExt, StreamSession, Supervisor, SupervisorSession};
use n9_core::chain::ReasoningFlow;
use n9_core::{ChatRequest, ChatResponse, RouterLink};
use std::collections::HashMap;
use ui9::names::Fqn;
use ui9_dui::{Act, Link, Pub};
use ui9_net::MeshNode;

pub struct ChatControlLoop {
    key: SessionKey,
    router: RouterLink,
    chat: Pub<ChatControl>,
    tracers: HashMap<Fqn, Address<TraceAgent>>,
}

impl ChatControlLoop {
    pub fn new(router: RouterLink, key: SessionKey) -> Self {
        Self {
            key: key.clone(),
            router,
            chat: Pub::new(key),
            tracers: HashMap::new(),
        }
    }
}

impl Supervisor for ChatControlLoop {
    type BasedOn = StreamSession<Self>;
    type GroupBy = ();
}

impl Agent for ChatControlLoop {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

impl ChatControlLoop {
    pub async fn create_tracer(&mut self, ctx: &mut Context<Self>) -> Result<Link<ReasoningFlow>> {
        let uuid = Uuid::new_v4();
        let fqn = self.key.push(uuid);
        let tracer = TraceAgent::new(fqn.clone());
        let addr: Address<_> = ctx.spawn_agent(tracer, ()).equip();
        addr.ping().await?;
        self.tracers.insert(fqn.clone(), addr);
        let peer_id = MeshNode::link()?.peer_id;
        Ok(Link::new(fqn, peer_id))
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
    async fn handle(&mut self, msg: SendRequest, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        // TODO: Add prompt to the turn
        self.chat.new_turn();

        let tracer = self.create_tracer(ctx).await?;
        self.chat.start_thinking(tracer.clone());

        let request = ChatRequest::user(&msg.prompt);
        let session = self.router.new_session_with_tracer(tracer).await?;
        let req = session.chat(request);
        self.chat.add(msg.prompt, Role::Request);

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
        let resp = msg.req.await?.squash();
        self.chat.add(resp, Role::Response);
        self.chat.stop_thinking();
        Ok(Next::events())
    }

    async fn fallback(&mut self, err: Error) -> Next<Self> {
        self.chat.stop_thinking();
        // TODO: Operation failure reporting here
        Next::events()
    }
}
