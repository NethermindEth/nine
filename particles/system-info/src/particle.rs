use crate::tools::ToolsList;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use crb::core::Slot;
use n9_core::{Particle, SubstanceBond, SubstanceLinks, Tool};

pub struct SystemInfo {
    substance: SubstanceLinks,
    bond: Slot<SubstanceBond<Self>>,
}

impl Particle for SystemInfo {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            bond: Slot::empty(),
        }
    }
}

impl Agent for SystemInfo {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for SystemInfo {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&ctx);
        bond.add_tool::<ToolsList>(self).await?;
        self.bond.fill(bond)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl Tool<ToolsList> for SystemInfo {
    fn name(&self) -> String {
        "tools_list".into()
    }

    async fn call_tool(
        &mut self,
        input: ToolsList,
        _ctx: &mut Context<Self>,
    ) -> Result<Vec<String>> {
        let tools = vec!["Tool 1".to_string()];
        Ok(tools)
    }
}
