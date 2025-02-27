use super::task::ChatTask;
use crate::tools::{TaskAdd, TaskDel, TaskId, TaskInfo, TaskParameters, TasksList};
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, DoAsync, Equip, Next};
use crb::core::Slot;
use crb::superagent::{Supervisor, SupervisorSession};
use n9_core::{Particle, SubstanceBond, SubstanceLinks, Tool, ToolInfo};
use typed_slab::TypedSlab;
use ui9_dui::Operation;

pub struct TaskRecord {
    parameters: TaskParameters,
    address: Address<ChatTask>,
}

impl From<(usize, &TaskRecord)> for TaskInfo {
    fn from((id, record): (usize, &TaskRecord)) -> Self {
        Self {
            id,
            parameters: record.parameters.clone(),
        }
    }
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
        let tasks = self.tasks.iter().map(TaskInfo::from).collect();
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
        Operation::event(format!("A task has added with id: {id}"));
        Ok(id)
    }
}

#[async_trait]
impl Tool<TaskDel> for ControlTask {
    async fn call_tool(&mut self, input: TaskDel, _ctx: &mut Context<Self>) -> Result<bool> {
        if let Some(id) = input.id {
            // Interrupt a specific task
            let Some(record) = self.tasks.remove(id) else {
                return Ok(false);
            };
            record.address.interrupt().ok();
        } else {
            // Interrupt all tasks
            for record in self.tasks.drain() {
                record.address.interrupt().ok();
            }
        }
        Ok(true)
    }
}
