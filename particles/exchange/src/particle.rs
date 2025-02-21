use crate::nominals::*;
use crb::agent::Agent;
use n9_core::{SubstanceBond, Tool};
use n9_kit::{LiquidParticle, Toolkit};

pub type ExchangeParticle = LiquidParticle<ExchangeToolkit>;

#[derive(Default)]
pub struct ExchangeToolkit;

impl<P> Toolkit<P> for ExchangeToolkit
where
    P: Agent,
    P: Tool<Price>,
{
    fn add_tools(&mut self, particle: &mut P, bond: &mut SubstanceBond<P>) {
        bond.add_tool::<Price>(particle);
    }
}
