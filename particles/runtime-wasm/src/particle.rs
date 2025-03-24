use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use n9_core::{Particle, SubstanceBond, SubstanceLinks, Tool};

pub struct RuntimeWasm {
    substance: SubstanceLinks,
}

impl Particle for RuntimeWasm {
    fn construct(substance: SubstanceLinks) -> Self {
        Self { substance }
    }
}

impl Agent for RuntimeWasm {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for RuntimeWasm {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        // TODO: Set bonds only if the wasm code does it
        Ok(Next::events())
    }
}
