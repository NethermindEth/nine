use crate::loader::ConfigLoader;
use crate::state::ConfigState;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use crb::superagent::{Supervisor, SupervisorSession};
use n9_node::{AtomId, Pub};

pub struct ConfigToml {
    state: Pub<ConfigState>,
}

impl ConfigToml {
    pub fn new() -> Self {
        let id = AtomId::local("@n9-config-toml");
        Self {
            state: Pub::connect(id),
        }
    }
}

impl Supervisor for ConfigToml {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for ConfigToml {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for ConfigToml {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let loader = ConfigLoader::new();
        ctx.spawn_agent(loader, ());
        Ok(Next::events())
    }
}
