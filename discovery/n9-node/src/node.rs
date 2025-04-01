use crate::publisher::{HubServer, HubServerLink};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, DoAsync, Equip, Next, ToAddress};
use crb::superagent::{Stacker, Supervisor, SupervisorSession};
use std::sync::OnceLock;

static NODE: OnceLock<NodeLink> = OnceLock::new();

pub struct NodeLink {
    pub node: Address<Node>,
    pub server: HubServerLink,
}

pub struct Node {}

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
        let mut stacker = Stacker::new();

        let server = HubServer::new();
        let server = stacker.schedule(server, ());

        let link = NodeLink {
            node: ctx.to_address(),
            server: server.equip(),
        };
        NODE.set(link)
            .map_err(|_| anyhow!("Node is already activated"))?;

        stacker.spawn_scheduled(ctx);

        Ok(Next::events())
    }
}
