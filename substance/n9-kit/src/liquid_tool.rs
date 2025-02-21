use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use crb::core::Slot;
use n9_core::{Particle, Prompt, SubstanceBond, SubstanceLinks, Tool};
use std::marker::PhantomData;

pub trait Toolkit<P: Agent>: Default + Send + 'static {
    fn add_tools(&mut self, particle: &mut P, bond: &mut SubstanceBond<P>);
}

pub struct LiquidParticle<K: Toolkit<Self>> {
    substance: SubstanceLinks,
    toolkit: PhantomData<K>,
    bond: Slot<SubstanceBond<Self>>,
}

impl<K> Particle for LiquidParticle<K>
where
    K: Toolkit<Self>,
{
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            toolkit: PhantomData,
            bond: Slot::empty(),
        }
    }
}

impl<K> Agent for LiquidParticle<K>
where
    K: Toolkit<Self>,
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
    K: Toolkit<Self>,
{
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&ctx);
        let mut toolkit = K::default();
        toolkit.add_tools(self, &mut bond);
        self.bond.fill(bond)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl<K, P> Tool<P> for LiquidParticle<K>
where
    K: Toolkit<Self>,
    P: Prompt,
{
    async fn call_tool(&mut self, input: P, _ctx: &mut Context<Self>) -> Result<P::Output> {
        Err(anyhow!("Not implemented"))
    }
}
