use crate::nominals::*;
use crb::agent::Agent;
use n9_core::{SubstanceBond, Tool};
use n9_kit::{LiquidParticle, Toolkit};

pub type ExchangeParticle = LiquidParticle<ExchangeToolkit>;

#[derive(Default)]
pub struct ExchangeToolkit;

impl Toolkit for ExchangeToolkit {
    fn add_tools(
        &mut self,
        particle: &mut LiquidParticle<Self>,
        bond: &mut SubstanceBond<LiquidParticle<Self>>,
    ) {
        bond.add_tool::<Price>(particle);
    }
}
