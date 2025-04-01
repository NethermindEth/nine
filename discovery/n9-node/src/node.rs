use crate::connector::{Connector, ConnectorLink, Key, PeerId};
use crate::publisher::{HubServer, HubServerLink};
use crate::subscriber::HubClient;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{
    Address, Agent, AgentSession, Context, DoAsync, Equip, Next, Standalone, ToAddress,
};
use crb::superagent::{PingExt, Stacker, Supervisor, SupervisorSession};
use std::sync::OnceLock;

static NODE: OnceLock<NodeLink> = OnceLock::new();

pub struct NodeLink {
    pub peer: PeerId,
    pub node: Address<Node>,
    pub server: HubServerLink,
    pub connector: ConnectorLink,
}

pub struct Node {}

impl Node {
    fn new() -> Self {
        Self {}
    }

    pub fn link() -> Result<&'static NodeLink> {
        NODE.get().ok_or_else(|| anyhow!("Node is not assigned"))
    }

    pub async fn bootstrap() -> Result<()> {
        let node = Self::new();
        node.spawn().ping().await?;
        Ok(())
    }

    pub async fn shutdown() -> Result<()> {
        if let Some(link) = NODE.get() {
            let mut node = link.node.clone();
            node.interrupt()?;
            node.join().await?;
        }
        Ok(())
    }
}

impl Standalone for Node {}

impl Supervisor for Node {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for Node {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Node {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let key = Key::generate();
        let peer = key.peer;

        let mut stacker = Stacker::new();

        let connector = Connector::new(key);
        let connector = stacker.schedule(connector, ());

        let server = HubServer::new();
        let server = stacker.schedule(server, ());

        let client = HubClient::new();
        let _client = stacker.schedule(client, ());

        let link = NodeLink {
            peer,
            node: ctx.to_address(),
            server: server.equip(),
            connector: connector.equip(),
        };
        NODE.set(link)
            .map_err(|_| anyhow!("Node is already activated"))?;

        stacker.spawn_scheduled(ctx);

        Ok(Next::events())
    }
}
