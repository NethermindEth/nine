use crate::tools::Price;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::Agent;
use n9_core::{SubstanceBond, Tool};
use n9_kit::{LiquidParticle, Toolkit};

pub type ExchangeParticle = LiquidParticle<ExchangeToolkit>;

#[derive(Default)]
pub struct ExchangeToolkit;

#[async_trait]
impl Toolkit for ExchangeToolkit {
    async fn add_tools(
        &mut self,
        particle: &mut LiquidParticle<Self>,
        bond: &mut SubstanceBond<LiquidParticle<Self>>,
    ) -> Result<()> {
        bond.add_tool::<Price>(particle).await?;
        Ok(())
    }
}
