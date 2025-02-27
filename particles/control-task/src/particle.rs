use crate::tools::{TasksList, TaskInfo};
use anyhow::Result;
use async_trait::async_trait;
use n9_core::{SubstanceLinks, SubstanceBond, Particle, Tool, ToolInfo};
use crb::core::Slot;
use crb::agent::{Agent, AgentSession, Context, Next, DoAsync};

pub struct ControlTask {
    substance: SubstanceLinks,
    bond: Slot<SubstanceBond<Self>>,
    // TODO: Add tasks flow (tracer) here
}

impl Particle for ControlTask {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            bond: Slot::empty(),
        }
    }
}

impl Agent for ControlTask {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for ControlTask {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&ctx);
        bond.add_tool::<TasksList>(self).await?;
        self.bond.fill(bond)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl Tool<TasksList> for ControlTask {
    fn name(&self) -> String {
        "tasks_list".into()
    }

    async fn call_tool(
        &mut self,
        input: TasksList,
        _ctx: &mut Context<Self>,
    ) -> Result<Vec<TaskInfo>> {
        Ok(Vec::new())
    }
}
