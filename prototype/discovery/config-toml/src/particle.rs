use crate::loader::{ConfigLoader, ConfigUpdates, NewConfig};
use crate::state::{ConfigDelta, ConfigQuery, ConfigState};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Equip, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{StreamSession, SubscribeExt, Supervisor, SupervisorSession};
use n9_node::{AtomId, Pub, PubEvent, PubValue, StateId};
use std::collections::HashMap;
use toml::{Table, Value};

struct Record {
    namespace: String,
    template: Value,
}

pub struct ConfigToml {
    state: Pub<ConfigState>,
    loader: Slot<Address<ConfigLoader>>,
    subscribers: HashMap<StateId, Record>,
    config: Value,
}

impl ConfigToml {
    pub fn new() -> Self {
        let id = AtomId::local("@n9-config-toml");
        Self {
            state: Pub::connect(id),
            loader: Slot::empty(),
            subscribers: HashMap::new(),
            config: table(),
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
        let actor = ConfigLoader::new();
        let loader = ctx.spawn_agent(actor, ()).0;
        let event = ConfigUpdates::for_listener(&ctx);
        let state_entry = loader.subscribe(event).await?;
        self.config = state_entry.state;
        self.loader.fill(loader)?;

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
                    } => {
                        let record = Record {
                            namespace,
                            template,
                        };
                        self.subscribers.insert(id, record);
                        self.distribute(id)?;
                    }
                }
            }
            PubValue::Disconnected => {
                self.subscribers.remove(&id);
            }
        }
        Ok(())
    }
}

impl ConfigToml {
    fn distribute(&self, id: StateId) -> Result<()> {
        let record = self
            .subscribers
            .get(&id)
            .ok_or_else(|| anyhow!("No state with id {id}"))?;
        let ns = &record.namespace;
        let config = get_nested_value(&self.config, ns)
            .ok_or_else(|| anyhow!("No value with path {ns} in a config."))?;
        let delta = ConfigDelta::NewValue {
            config: config.clone(),
        };
        self.state.direct(id, delta)?;
        Ok(())
    }
}

fn get_nested_value<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').fold(Some(value), |acc, key| acc?.get(key))
}

#[async_trait]
impl OnEvent<NewConfig> for ConfigToml {
    async fn handle(&mut self, query: NewConfig, ctx: &mut Context<Self>) -> Result<()> {
        for id in self.subscribers.keys() {
            // TODO: Absorb errors (a special macro)
            self.distribute(*id).ok();
        }
        Ok(())
    }
}

fn table() -> Value {
    Value::Table(Table::new())
}
