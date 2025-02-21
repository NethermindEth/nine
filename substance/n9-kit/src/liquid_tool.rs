use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context};
use n9_core::{Tool, ToolInput, SubstanceLinks, Particle};

pub trait LiquidTool {
}

pub struct LiquidParticle {
    substance: SubstanceLinks,
}

impl Particle for LiquidParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
        }
    }
}

impl Agent for LiquidParticle {
    type Context = AgentSession<Self>;
}

#[async_trait]
impl<T> Tool<T> for LiquidParticle
where T: ToolInput,
{
    async fn call_tool(&mut self, input: T, _ctx: &mut Context<Self>) -> Result<T::ToolOutput> {
        Err(anyhow!("Not implemented"))
    }
}
