use super::SubstanceLinks;
use crate::router::{
    keeper::{ConfigSegmentUpdates, Keeper, UpdateConfig},
    model::Model,
    tool::{Prompt, Tool},
    types::ToolMeta,
};
use crate::Config;
use anyhow::Result;
use crb::agent::{Address, Agent, ToAddress};
use crb::superagent::Entry;

impl SubstanceLinks {
    pub fn bond<A: Agent>(&mut self, recipient: impl ToAddress<A>) -> SubstanceBond<A> {
        SubstanceBond {
            address: recipient.to_address(),
            substance: self.clone(),
        }
    }
}

pub trait Particle: Agent<Context: Default> {
    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn construct(substance: SubstanceLinks) -> Self;
}

pub struct SubstanceBond<A: Agent> {
    address: Address<A>,
    substance: SubstanceLinks,
}

impl<A: Agent> SubstanceBond<A> {
    pub async fn live_config_updates<C>(&mut self) -> Result<(C, Entry<ConfigSegmentUpdates>)>
    where
        A: UpdateConfig<C>,
        C: Config,
    {
        let keeper = self.substance.router.get_keeper().await?;
        keeper.live_config_updates(&self.address).await
    }

    pub fn add_keeper(&mut self) -> Result<()>
    where
        A: Keeper,
    {
        let address = self.address.clone();
        self.substance.router.add_keeper(address)
    }

    pub fn add_model(&mut self) -> Result<()>
    where
        A: Model,
    {
        let address = self.address.clone();
        self.substance.router.add_model(address)
    }

    pub async fn add_tool<P>(&mut self, tool: &A) -> Result<()>
    where
        A: Tool<P>,
        P: Prompt,
    {
        let address = self.address.clone();
        let meta = ToolMeta {
            name: tool.name(),
            description: tool.description(),
            parameters: tool.parameters(),
        };
        self.substance.router.add_tool(address, meta).await?;
        Ok(())
    }
}
