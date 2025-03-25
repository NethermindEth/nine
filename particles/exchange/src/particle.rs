use crate::tools;
use anyhow::Result;
use async_trait::async_trait;
use n9_core::SubstanceBond;
use n9_kit::{LiquidParticle, Toolkit};

static TOOLKIT: &str = "Exchange";

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
        bond.add_tool::<tools::Price>(particle, TOOLKIT, "Get Price")
            .await?;
        bond.add_tool::<tools::Tickers>(particle, TOOLKIT, "List Of Tickers")
            .await?;
        bond.add_tool::<tools::Order>(particle, TOOLKIT, "Place An Order")
            .await?;
        Ok(())
    }
}
