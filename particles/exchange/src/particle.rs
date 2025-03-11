use crate::tools;
use anyhow::Result;
use async_trait::async_trait;
use n9_core::SubstanceBond;
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
        bond.add_tool::<tools::Price>(particle).await?;
        bond.add_tool::<tools::Tickers>(particle).await?;
        bond.add_tool::<tools::Order>(particle).await?;
        Ok(())
    }
}
