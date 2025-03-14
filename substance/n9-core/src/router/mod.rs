pub mod keeper;
pub mod model;
pub mod tool;
pub mod types;

use crate::chain::{ReasoningFlow, ReasoningSession, SessionLink};
use crate::tracers::tools::Tools;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, Equip, Next};
use crb::superagent::{InteractExt, OnRequest, Request, Responder, Supervisor, SupervisorSession};
use derive_more::{Deref, DerefMut, From, Into};
use keeper::KeeperLink;
use model::ModelLink;
use std::collections::HashMap;
use tool::ToolRecord;
use typed_slab::TypedSlab;
use types::{ChatResponse, ToolId};
use ui9_dui::{Link, Pub};

#[derive(From, Into)]
pub struct ReqId(usize);

#[derive(Deref, DerefMut, From, Clone)]
pub struct RouterLink {
    address: Address<ReasoningRouter>,
}

pub struct ReasoningRouter {
    models: Vec<ModelLink>,
    keepers: Vec<KeeperLink>,
    tools: HashMap<ToolId, ToolRecord>,
    requests: TypedSlab<ReqId, Responder<ChatResponse>>,
    tools_pub: Pub<Tools>,
}

impl ReasoningRouter {
    pub fn new() -> Self {
        Self {
            models: Vec::default(),
            keepers: Vec::default(),
            tools: HashMap::default(),
            requests: TypedSlab::default(),
            tools_pub: Pub::unified(),
        }
    }
}

impl Supervisor for ReasoningRouter {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for ReasoningRouter {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::events()
    }
}

impl RouterLink {
    pub async fn new_session(&self) -> Result<SessionLink> {
        let msg = NewSession { tracer: None };
        self.interact(msg).await.map_err(Error::from)
    }

    pub async fn new_session_with_tracer(
        &self,
        tracer: Link<ReasoningFlow>,
    ) -> Result<SessionLink> {
        let msg = NewSession {
            tracer: Some(tracer),
        };
        self.interact(msg).await.map_err(Error::from)
    }
}

struct NewSession {
    tracer: Option<Link<ReasoningFlow>>,
}

impl Request for NewSession {
    type Response = SessionLink;
}

#[async_trait]
impl OnRequest<NewSession> for ReasoningRouter {
    async fn on_request(&mut self, _: NewSession, ctx: &mut Context<Self>) -> Result<SessionLink> {
        let link = ctx.equip();
        let session = ReasoningSession::new(link);
        let addr = ctx.spawn_agent(session, ());
        Ok(addr.equip())
    }
}
