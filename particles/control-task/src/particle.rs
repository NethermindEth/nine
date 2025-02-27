use crate::tools::{TaskAdd, TaskDel, TaskId, TaskInfo, TasksList, TaskParameters};
use super::task::ChatTask;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, Address, Equip};
use crb::superagent::{SupervisorSession, Supervisor};
use crb::core::Slot;
use n9_core::{Particle, SubstanceBond, SubstanceLinks, Tool, ToolInfo};
use ui9_dui::Operation;
use typed_slab::TypedSlab;

pub struct TaskRecord {
    parameters: TaskParameters,
    address: Address<ChatTask>,
}

pub struct ControlTask {
    substance: SubstanceLinks,
    bond: Slot<SubstanceBond<Self>>,
    tasks: TypedSlab<TaskId, TaskRecord>,
    // TODO: Add tasks flow (tracer) here
}

impl Particle for ControlTask {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            bond: Slot::empty(),
            tasks: TypedSlab::new(),
        }
    }
}

impl Supervisor for ControlTask {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for ControlTask {
    type Context = SupervisorSession<Self>;

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
        bond.add_tool::<TaskAdd>(self).await?;
        bond.add_tool::<TaskDel>(self).await?;
        self.bond.fill(bond)?;
        Ok(Next::events())
    }
}

#[async_trait]
impl Tool<TasksList> for ControlTask {
    async fn call_tool(
        &mut self,
        input: TasksList,
        _ctx: &mut Context<Self>,
    ) -> Result<Vec<TaskInfo>> {
        let tasks = self.tasks.iter()
            .map(|(id, record)| {
                TaskInfo {
                    id,
                    parameters: record.parameters.clone(),
                }
            })
            .collect();
        Ok(tasks)
    }
}

#[async_trait]
impl Tool<TaskAdd> for ControlTask {
    async fn call_tool(&mut self, input: TaskAdd, ctx: &mut Context<Self>) -> Result<TaskId> {
        let parameters = TaskParameters::from(input);
        let chat_task = ChatTask::new(parameters.clone());
        let address = ctx.spawn_agent(chat_task, ()).equip();
        let record = TaskRecord {
            parameters,
            address,
        };
        let id = self.tasks.insert(record);
        Operation::event("A task has added with id: {id}");
        Ok(id)
    }
}

#[async_trait]
impl Tool<TaskDel> for ControlTask {
    async fn call_tool(&mut self, input: TaskDel, _ctx: &mut Context<Self>) -> Result<bool> {
        if let Some(record) = self.tasks.remove(input.id) {
            record.address.interrupt();
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
