use crate::flow::Tasks;
use crate::task::ChatTask;
use crate::tools::{TaskAdd, TaskDel, TaskId, TaskInfo, TaskParameters, TasksList};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, DoAsync, Equip, Next};
use crb::core::Slot;
use crb::superagent::{Supervisor, SupervisorSession};
use n9_core::{CallMeta, Particle, SubstanceBond, SubstanceLinks, Tool};
use std::collections::HashMap;
use ui9::names::Fqn;
use ui9_dui::{Operation, Pub};

pub struct TaskRecord {
    parameters: TaskParameters,
    address: Address<ChatTask>,
}

impl From<(&usize, &TaskRecord)> for TaskInfo {
    fn from((id, record): (&usize, &TaskRecord)) -> Self {
        Self {
            id: *id,
            parameters: record.parameters.clone(),
        }
    }
}

#[derive(Default)]
struct ScopeRecord {
    tasks: HashMap<TaskId, TaskRecord>,
}

pub struct ControlTask {
    substance: SubstanceLinks,
    bond: Slot<SubstanceBond<Self>>,
    task_id: usize,
    scopes: HashMap<Fqn, ScopeRecord>,
    state: Pub<Tasks>,
    // TODO: Add tasks flow (tracer) here
}

impl Particle for ControlTask {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            bond: Slot::empty(),
            task_id: 0,
            scopes: HashMap::new(),
            state: Pub::unified(),
        }
    }
}

impl Supervisor for ControlTask {
    type BasedOn = AgentSession<Self>;
    type GroupBy = Fqn;
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
    async fn call_tool_meta(
        &mut self,
        input: TasksList,
        meta: CallMeta,
        ctx: &mut Context<Self>,
    ) -> Result<Vec<TaskInfo>> {
        let Some(scope) = self.scopes.get(&meta.chat) else {
            return Ok(Vec::new());
        };
        let tasks = scope.tasks.iter().map(TaskInfo::from).collect();
        Ok(tasks)
    }
}

#[async_trait]
impl Tool<TaskAdd> for ControlTask {
    async fn call_tool_meta(
        &mut self,
        input: TaskAdd,
        meta: CallMeta,
        ctx: &mut Context<Self>,
    ) -> Result<TaskId> {
        let chat_id = meta.chat;
        self.task_id += 1;
        let id = self.task_id;
        let parameters = TaskParameters::from(input);
        let state = self.state.create(id, parameters.clone());
        let chat_task = ChatTask::new(state, chat_id.clone());
        let address = ctx.spawn_agent(chat_task, chat_id.clone()).equip();
        let record = TaskRecord {
            parameters: parameters.clone(),
            address,
        };
        self.scopes
            .entry(chat_id)
            .or_default()
            .tasks
            .insert(id, record);
        Operation::event(format!("A task has added with id: {id}"));
        Ok(id)
    }
}

#[async_trait]
impl Tool<TaskDel> for ControlTask {
    async fn call_tool_meta(
        &mut self,
        input: TaskDel,
        meta: CallMeta,
        ctx: &mut Context<Self>,
    ) -> Result<bool> {
        let Some(scope) = self.scopes.get_mut(&meta.chat) else {
            return Err(anyhow!(
                "This chat doesn't have a tasks scope to work with."
            ));
        };
        if let Some(id) = input.id {
            // Interrupt a specific task
            let Some(record) = scope.tasks.remove(&id) else {
                return Ok(false);
            };
            record.address.interrupt().ok();
        } else {
            // Interrupt all tasks
            for (_, record) in scope.tasks.drain() {
                record.address.interrupt().ok();
            }
        }
        Ok(true)
    }
}
