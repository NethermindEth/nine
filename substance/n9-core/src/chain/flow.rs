use crb::core::uuid::Uuid;
use derive_more::{Deref, DerefMut, Display, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ui9::names::Fqn;
use ui9_dui::flow::{Flow, Unified};
use ui9_dui::publisher::{Publisher, Tracer};
use ui9_dui::subscriber::{Listener, Subscriber};

#[derive(Deref, DerefMut, From, Into)]
pub struct OperationSub {
    listener: Listener<Operation>,
}

impl Subscriber for Operation {
    type Driver = OperationSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct OperationPub {
    tracer: Tracer<Operation>,
}

impl OperationPub {
    pub fn start(&mut self, model: String) {
        let event = OperationEvent::Start { model };
        self.event(event);
    }
}

impl Publisher for Operation {
    type Driver = OperationPub;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub operations: Vec<OperationInfo>,
}

impl Default for Operation {
    fn default() -> Self {
        Self {
            operations: Vec::new(),
        }
    }
}

impl Flow for Operation {
    type Event = OperationEvent;
    type Action = ();

    fn apply(&mut self, event: Self::Event) {
        match event {
            OperationEvent::Start { model } => {
                let info = OperationInfo { model };
                self.operations.push(info);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationEvent {
    Start { model: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationInfo {
    pub model: String,
}

/*
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
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
*/
