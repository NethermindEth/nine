use super::{ReasoningRouter, RouterLink};
use crate::Config;
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, Equip, MessageFor, OnEvent, ToAddress};
use crb::send::{Recipient, Sender};
use crb::superagent::{
    Entry, Fetcher, InteractExt, ManageSubscription, OnRequest, Request, SubscribeExt, Subscription,
};
use derive_more::{Deref, DerefMut};
use std::any::type_name;
use std::marker::PhantomData;
use std::sync::Arc;
use toml::Value;

pub trait Keeper: OnRequest<GetConfig> + ManageSubscription<ConfigSegmentUpdates> {}

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

pub trait KeeperAddress
where
    Self: InteractExt<GetConfig> + SubscribeExt<ConfigSegmentUpdates> + Send + Sync,
{
}

impl<T> KeeperAddress for T where
    Self: InteractExt<GetConfig> + SubscribeExt<ConfigSegmentUpdates> + Send + Sync
{
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

pub struct GetConfig;

impl Request for GetConfig {
    type Response = Value;
}

// Live updates

impl KeeperLink {
    pub async fn live_config_updates<A, C>(
        &self,
        address: impl ToAddress<A>,
    ) -> Result<(C, Entry<ConfigSegmentUpdates>)>
    where
        A: UpdateConfig<C>,
        C: Config,
    {
        let recipient = TypedConfigListener {
            recipient: address.to_address().sender(),
        };
        let updates = ConfigSegmentUpdates {
            get_config: GetConfig,
            recipient: Recipient::new(recipient),
        };
        let state_entry = self.subscribe(updates).await?;
        let config = state_entry.state.try_into()?;
        Ok((config, state_entry.entry))
    }
}

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

pub struct TypedConfigListener<C: Config> {
    recipient: Recipient<UpdateConfigEvent<C>>,
}

impl<C> Sender<NewConfigSegment> for TypedConfigListener<C>
where
    C: Config,
{
    fn send(&self, value: NewConfigSegment) -> Result<()> {
        let event = UpdateConfigEvent {
            _type: PhantomData::<C>,
            value: value.0,
        };
        self.recipient.send(event)?;
        Ok(())
    }
}

pub struct UpdateConfigEvent<C> {
    _type: PhantomData<C>,
    value: Value,
}

#[async_trait]
impl<A, C> MessageFor<A> for UpdateConfigEvent<C>
where
    A: UpdateConfig<C>,
    C: Config,
{
    async fn handle(self: Box<Self>, agent: &mut A, ctx: &mut Context<A>) -> Result<()> {
        let result = match self.value.try_into() {
            Ok(config) => agent.update_config(config, ctx).await,
            Err(err) => {
                let ns = C::NAMESPACE;
                log::error!("Can't parse the section 'particle.{ns}.config': {err}");
                Err(err.into())
            }
        };
        if let Err(err) = result {
            agent.fallback(err, ctx);
        }
        Ok(())
    }
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
