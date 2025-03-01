use super::{ReasoningRouter, RouterLink};
use crate::Config;
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Equip, OnEvent};
use crb::superagent::{Fetcher, InteractExt, OnRequest, Request};
use derive_more::{Deref, DerefMut};
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

impl KeeperLink {
    pub async fn get_config<C>(&self) -> Result<C>
    where
        C: Config,
    {
        let config = self.address.get_config().await?.try_into()?;
        Ok(config)
    }
}

#[async_trait]
pub trait KeeperAddress: Sync + Send {
    fn get_config(&self) -> Fetcher<Value>;
}

#[async_trait]
impl<M: Keeper> KeeperAddress for Address<M> {
    fn get_config(&self) -> Fetcher<Value> {
        self.interact(GetConfig)
    }
}

pub struct GetConfig;

impl Request for GetConfig {
    type Response = Value;
}

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
