use crate::loader::ConfigLoader;
use crate::state::{ConfigQuery, ConfigState};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::{StreamSession, Supervisor, SupervisorSession};
use n9_node::{AtomId, Pub, PubEvent, PubValue};

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
    type BasedOn = StreamSession<Self>;
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
        let queries = self.state.queries().await?;
        ctx.consume(queries);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<PubEvent<ConfigState>> for ConfigToml {
    async fn handle(
        &mut self,
        query: PubEvent<ConfigState>,
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        let id = query.from;
        match query.value {
            PubValue::Connected => {}
            PubValue::Query(query) => {
                //
                match query {
                    ConfigQuery::GetConfig {
                        namespace,
                        template,
                    } => {}
                }
            }
            PubValue::Disconnected => {}
        }
        Ok(())
    }
}
