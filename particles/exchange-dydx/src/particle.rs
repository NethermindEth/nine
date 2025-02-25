use crate::config::DyDxConfig;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use crb::core::Slot;
use crb::superagent::{Entry, Supervisor, SupervisorSession};
use dydx::indexer::IndexerClient;
use n9_core::{
    ConfigSegmentUpdates, Particle, Prompt, SubstanceBond, SubstanceLinks, Tool, ToolResponse,
    UpdateConfig,
};
use n9_exchange::tools::{Price, Tickers};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui9_dui::Operation;

pub struct DyDxParticle {
    substance: SubstanceLinks,
    config_updates: Option<Entry<ConfigSegmentUpdates>>,
    bond: Slot<SubstanceBond<Self>>,
    indexer: Slot<IndexerClient>,
}

impl Supervisor for DyDxParticle {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Particle for DyDxParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            config_updates: None,
            bond: Slot::empty(),
            indexer: Slot::empty(),
        }
    }
}

impl Agent for DyDxParticle {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for DyDxParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&ctx);

        let (config, entry) = bond.live_config_updates().await?;
        self.config_updates = Some(entry);
        self.update_config(config, ctx).await?;

        bond.add_tool::<Price>(self).await?;
        bond.add_tool::<Tickers>(self).await?;

        self.bond.fill(bond)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl UpdateConfig<DyDxConfig> for DyDxParticle {
    async fn update_config(&mut self, config: DyDxConfig, _ctx: &mut Context<Self>) -> Result<()> {
        let op = Operation::start("Configuring dYdX");
        let indexer = IndexerClient::new(config.config.indexer);
        self.indexer.refill(indexer);
        op.end();
        Ok(())
    }
}

#[async_trait]
impl Tool<Price> for DyDxParticle {
    fn name(&self) -> String {
        "dydx_price".into()
    }

    async fn call_tool(&mut self, input: Price, _ctx: &mut Context<Self>) -> Result<String> {
        let ticker = input.ticker.into();
        let price = self
            .indexer
            .get()?
            .markets()
            .get_perpetual_market(&ticker)
            .await?
            .oracle_price
            .map(|x| x.to_string())
            .unwrap_or_else(|| "No oracle price for the ticker.".to_string());
        Ok(price)
    }
}

#[async_trait]
impl Tool<Tickers> for DyDxParticle {
    fn name(&self) -> String {
        "dydx_tickers".into()
    }
}
