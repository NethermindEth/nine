use super::recorder::{Recorder, RecorderLink};
use crate::atom::Aqn;
use crate::publisher::State;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, OnEvent, RunAgent};
use crb::runtime::{InteractiveRuntime, Runtime};
use crb::superagent::{
    EventBridge, InteractExt, OnRequest, Request, StreamSession, Supervisor, SupervisorSession,
};
use derive_more::{Deref, DerefMut, From};
use std::collections::HashMap;
use std::sync::LazyLock;

static PUB_BRIDGE: LazyLock<EventBridge<Delegate>> = LazyLock::new(|| EventBridge::new());

#[derive(Deref, DerefMut, From, Clone)]
pub struct HubServerLink {
    address: Address<HubServer>,
}

impl HubServerLink {
    pub async fn discover(&self, aqn: Aqn) -> Result<RecorderLink> {
        let msg = Discover { aqn };
        let link = self.address.interact(msg).await?;
        Ok(link)
    }
}

impl HubServer {
    pub fn spawn_recorder<S>(aqn: Aqn, recorder: Recorder<S>) -> Address<Recorder<S>>
    where
        S: State,
    {
        let runtime = RunAgent::new(recorder);
        let address = runtime.address();
        let delegate = Delegate {
            aqn,
            link: RecorderLink::new(runtime.address().clone()),
            runtime: Box::new(runtime),
        };
        PUB_BRIDGE.send(delegate);
        address
    }
}

pub struct HubServer {
    recorders: HashMap<Aqn, RecorderLink>,
}

impl HubServer {
    pub fn new() -> Self {
        Self {
            recorders: HashMap::new(),
        }
    }
}

impl Supervisor for HubServer {
    type BasedOn = StreamSession<Self>;
    type GroupBy = ();
}

impl Agent for HubServer {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for HubServer {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let events = PUB_BRIDGE.events().await?;
        ctx.consume(events);
        Ok(Next::events())
    }
}

pub struct Delegate {
    aqn: Aqn,
    link: RecorderLink,
    runtime: Box<dyn Runtime>,
}

#[async_trait]
impl OnEvent<Delegate> for HubServer {
    async fn handle(&mut self, event: Delegate, ctx: &mut Context<Self>) -> Result<()> {
        let path = &event.aqn;
        if self.recorders.contains_key(path) {
            Err(anyhow!("Recorder {path} already registered"))
        } else {
            ctx.spawn_trackable(event.runtime, ());
            // self.relations.insert(rel, path.clone());
            self.recorders.insert(path.clone(), event.link);
            Ok(())
        }
    }
}

struct Discover {
    aqn: Aqn,
}

impl Request for Discover {
    type Response = RecorderLink;
}

#[async_trait]
impl OnRequest<Discover> for HubServer {
    async fn on_request(
        &mut self,
        req: Discover,
        _ctx: &mut Context<Self>,
    ) -> Result<RecorderLink> {
        self.recorders
            .get(&req.aqn)
            .cloned()
            .ok_or_else(|| anyhow!("Recorder {} not found", req.aqn))
    }
}
