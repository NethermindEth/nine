use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use crb::core::Slot;
use n9_core::{Particle, Prompt, SubstanceBond, SubstanceLinks, Tool};

pub trait Toolkit: Default + Send + 'static {
    fn add_tools<P>(&mut self, bond: &mut SubstanceBond<P>)
    where
        P: Agent;
}

pub struct LiquidParticle<K: Toolkit> {
    substance: SubstanceLinks,
    toolkit: K,
    bond: Slot<SubstanceBond<Self>>,
}

impl<K> Particle for LiquidParticle<K>
where
    K: Toolkit,
{
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            toolkit: K::default(),
            bond: Slot::empty(),
        }
    }
}

impl<K> Agent for LiquidParticle<K>
where
    K: Toolkit,
{
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl<K> DoAsync<Initialize> for LiquidParticle<K>
where
    K: Toolkit,
{
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&ctx);
        self.toolkit.add_tools(&mut bond);
        self.bond.fill(bond)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl<K, P> Tool<P> for LiquidParticle<K>
where
    K: Toolkit,
    P: Prompt,
{
    async fn call_tool(&mut self, input: P, _ctx: &mut Context<Self>) -> Result<P::Output> {
        Err(anyhow!("Not implemented"))
    }
}
