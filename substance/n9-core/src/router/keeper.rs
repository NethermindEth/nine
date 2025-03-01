use super::{ReasoningRouter, RouterLink};
use crate::Config;
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Equip, OnEvent};
use crb::send::Recipient;
use crb::superagent::{Fetcher, InteractExt, OnRequest, Request, Subscription};
use derive_more::{Deref, DerefMut};
use std::any::type_name;
use std::sync::Arc;
use toml::Value;

pub trait Keeper: OnRequest<GetConfig> {}

#[derive(Deref, DerefMut, Clone)]
pub struct KeeperLink {
    address: Arc<dyn KeeperAddress>,
}

impl<M: Keeper> From<Address<M>> for KeeperLink {
    fn from(addr: Address<M>) -> Self {
        Self {
            address: Arc::new(addr),
        }
    }
}

// Single request

impl KeeperLink {
    pub async fn get_config<C>(&self) -> Result<C>
    where
        C: Config,
    {
        let config = self.address.interact(GetConfig).await?.try_into()?;
        Ok(config)
    }
}

pub trait KeeperAddress: InteractExt<GetConfig> + Send + Sync {}

impl<T> KeeperAddress for T where Self: InteractExt<GetConfig> + Send + Sync {}

pub struct GetConfig;

impl Request for GetConfig {
    type Response = Value;
}

// Live updates

#[async_trait]
pub trait UpdateConfig<C: Config>: Agent {
    async fn update_config(&mut self, config: C, ctx: &mut Context<Self>) -> Result<()>;

    fn fallback(&mut self, err: Error, _ctx: &mut Context<Self>) {
        let typ = type_name::<C>();
        log::error!("Can't load the config {typ}: {err}");
    }
}

pub struct NewConfigSegment(pub Value);

pub struct ConfigSegmentUpdates {
    get_config: GetConfig,
    // TODO: Keep `Arc` with a default value here
    recipient: Recipient<NewConfigSegment>,
}

impl Subscription for ConfigSegmentUpdates {
    type State = Value;
}

// Config registry

impl RouterLink {
    pub fn add_keeper<K>(&mut self, addr: Address<K>) -> Result<()>
    where
        K: Keeper,
    {
        let msg = AddKeeper { link: addr.equip() };
        self.address.event(msg)?;
        Ok(())
    }

    pub async fn get_keeper(&mut self) -> Result<KeeperLink> {
        self.interact(GetKeeper).await.map_err(Error::from)
    }
}

pub struct AddKeeper {
    link: KeeperLink,
}

#[async_trait]
impl OnEvent<AddKeeper> for ReasoningRouter {
    async fn handle(&mut self, msg: AddKeeper, _ctx: &mut Context<Self>) -> Result<()> {
        self.keepers.push(msg.link);
        Ok(())
    }
}

struct GetKeeper;

impl Request for GetKeeper {
    type Response = KeeperLink;
}

#[async_trait]
impl OnRequest<GetKeeper> for ReasoningRouter {
    async fn on_request(&mut self, _: GetKeeper, _ctx: &mut Context<Self>) -> Result<KeeperLink> {
        self.keepers
            .first()
            .cloned()
            .ok_or_else(|| anyhow!("Keepers are not installed"))
    }
}
