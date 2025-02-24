use crb::agent::{Agent, AgentSession};
use n9_core::{Particle, SubstanceLinks};

pub struct SystemInfo {
    substance: SubstanceLinks,
}

impl Particle for SystemInfo {
    fn construct(substance: SubstanceLinks) -> Self {
        Self { substance }
    }
}

impl Agent for SystemInfo {
    type Context = AgentSession<Self>;
}
