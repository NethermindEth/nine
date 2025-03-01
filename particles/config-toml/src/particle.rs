use crate::config_loader::{
    merge_configs, table, wrap_level, ConfigLoader, ConfigUpdates, NewConfig, StoreTemplate,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::core::{Slot, Unique};
use crb::send::Sender;
use crb::superagent::{
    Entry, ManageSubscription, OnRequest, SubscribeExt, Supervisor, SupervisorSession,
};
use derive_more::From;
use n9_core::{
    ConfigSegmentUpdates, GetConfig, Keeper, NewConfigSegment, Particle, SubstanceBond,
    SubstanceLinks,
};
use std::collections::HashMap;
use toml::Value;

pub struct TomlConfigParticle {
    // TODO: Use bond only
    substance: SubstanceLinks,
    bond: Slot<SubstanceBond<Self>>,

    config: MergedConfig,
    updater: Slot<Entry<ConfigUpdates>>,
    subscribers: HashMap<Unique<ConfigSegmentUpdates>, Subscriber>,
    loader: Slot<Address<ConfigLoader>>,
}

impl Particle for TomlConfigParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            bond: Slot::empty(),

            config: MergedConfig::new(),
            updater: Slot::empty(),
            subscribers: HashMap::new(),
            loader: Slot::empty(),
        }
    }
}

impl Supervisor for TomlConfigParticle {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for TomlConfigParticle {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

impl Keeper for TomlConfigParticle {}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for TomlConfigParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&ctx);
        bond.add_keeper()?;
        self.bond.fill(bond)?;

        let loader = ConfigLoader::new();
        let (addr, _) = ctx.spawn_agent(loader, ());
        let sub = ConfigUpdates::for_listener(&ctx);
        let state_entry = addr.subscribe(sub).await?;
        self.loader.fill(addr)?;

        // No subscribers here, not necessary to distribute the config
        self.config = MergedConfig::from(state_entry.state);
        self.updater.fill(state_entry.entry)?;

        Ok(Next::events())
    }
}

#[async_trait]
impl OnRequest<GetConfig> for TomlConfigParticle {
    async fn on_request(&mut self, msg: GetConfig, _: &mut Context<Self>) -> Result<Value> {
        let config = self.config.get_config_segment(&msg);
        Ok(config)
    }
}

pub struct Subscriber {
    last_value: Option<Value>,
}

impl TomlConfigParticle {
    pub fn distribute(&mut self) {
        for (id, subscriber) in &mut self.subscribers {
            let value = self.config.get_config_segment(&id.get_config);
            if subscriber.last_value.as_ref() == Some(&value) {
                subscriber.last_value = Some(value.clone());
            }
            id.recipient.send(NewConfigSegment(value.clone())).ok();
        }
    }
}

impl TomlConfigParticle {
    fn merged_template(&self) -> Value {
        let mut particles = table();
        for (id, _) in &self.subscribers {
            let template = id.get_config.template.clone();
            let config = wrap_level("config", template);
            let scope = &id.get_config.namespace;
            let scoped = wrap_level(scope, config);
            merge_configs(&mut particles, &scoped);
        }
        wrap_level("particle", particles)
    }
}

#[async_trait]
impl ManageSubscription<ConfigSegmentUpdates> for TomlConfigParticle {
    async fn subscribe(
        &mut self,
        sub_id: Unique<ConfigSegmentUpdates>,
        _ctx: &mut Context<Self>,
    ) -> Result<Value> {
        let subscriber = Subscriber { last_value: None };
        let value = self.config.get_config_segment(&sub_id.get_config);
        self.subscribers.insert(sub_id, subscriber);

        let template = self.merged_template();
        let msg = StoreTemplate(template);
        self.loader.get()?.event(msg)?;
        Ok(value)
    }

    async fn unsubscribe(
        &mut self,
        sub_id: Unique<ConfigSegmentUpdates>,
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        self.subscribers.remove(&sub_id);
        Ok(())
    }
}

#[derive(From)]
pub struct MergedConfig {
    value: Value,
}

impl MergedConfig {
    fn new() -> Self {
        Self { value: table() }
    }
}

impl MergedConfig {
    fn get_config_segment(&self, seg: &GetConfig) -> Value {
        self.get_config_segment_opt(seg)
            .unwrap_or_else(|| seg.template.clone())
    }

    fn get_config_segment_opt(&self, seg: &GetConfig) -> Option<Value> {
        self.value
            .get("particle")?
            .get(&seg.namespace)?
            .get("config")
            .cloned()
    }
}

#[async_trait]
impl OnEvent<NewConfig> for TomlConfigParticle {
    async fn handle(&mut self, config: NewConfig, _ctx: &mut Context<Self>) -> Result<()> {
        self.config = MergedConfig::from(config.0);
        self.distribute();
        Ok(())
    }
}
