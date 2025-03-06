use crate::flow::{Flow, Unified};
use crate::publisher::{Publisher, Tracer};
use crate::subscriber::{Listener, Subscriber};
use crb::core::uuid::Uuid;
use derive_more::{Deref, DerefMut, From, Into, Display};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ui9::names::Fqn;

#[derive(Deref, DerefMut, From, Into)]
pub struct JobSub {
    listener: Listener<Job>,
}

impl Subscriber for Job {
    type Driver = JobSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct JobPub {
    tracer: Tracer<Job>,
}

impl Publisher for Job {
    type Driver = JobPub;
}

impl Unified for Job {
    fn fqn() -> Fqn {
        Fqn::root("@job")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub operations: BTreeMap<OperationId, OperationRecord>,
}

impl Default for Job {
    fn default() -> Self {
        Self {
            operations: BTreeMap::new(),
        }
    }
}

impl Flow for Job {
    type Event = JobData;
    type Action = JobData;

    fn apply(&mut self, event: Self::Event) {
        match event {
            JobData::Begin { id, task } => {
                let record = OperationRecord { task: task.into() };
                self.operations.insert(id, record);
            }
            JobData::End { id } => {
                self.operations.remove(&id);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobData {
    Begin { id: OperationId, task: String },
    End { id: OperationId },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct OperationId {
    id: Uuid,
}

impl OperationId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    pub task: String,
}
