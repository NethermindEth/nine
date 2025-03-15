use crate::router::types::{ToolingChatRequest, ToolingChatResponse};
use chrono::NaiveDateTime;
use crb::core::uuid::Uuid;
use derive_more::{Deref, DerefMut, Display, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ui9::names::Fqn;
use ui9_dui::flow::{Flow, Unified};
use ui9_dui::publisher::{Publisher, Tracer};
use ui9_dui::subscriber::{Listener, Subscriber};
#[derive(Deref, DerefMut, From, Into)]
pub struct ReasoningSub {
    listener: Listener<ReasoningFlow>,
}

impl ReasoningSub {
    pub fn operation(&self, info: OperationInfo) {
        let action = ReasoningAction::Add(info);
        self.action(action);
    }
}

impl Subscriber for ReasoningFlow {
    type Driver = ReasoningSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct ReasoningPub {
    tracer: Tracer<ReasoningFlow>,
}

impl ReasoningPub {
    pub fn operation(&self, info: OperationInfo) {
        let event = ReasoningEvent::Add(info);
        self.event(event);
    }
}

impl Publisher for ReasoningFlow {
    type Driver = ReasoningPub;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningFlow {
    pub operations: Vec<OperationInfo>,
    pub operation: Option<ReasoningOperation>,
}

impl Default for ReasoningFlow {
    fn default() -> Self {
        Self {
            operations: Vec::new(),
            operation: None,
        }
    }
}

impl Flow for ReasoningFlow {
    type Event = ReasoningEvent;
    type Action = ReasoningAction;

    fn apply(&mut self, event: Self::Event) {
        // self.operations.push(event);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationInfo {
    pub id: Uuid,
    pub timestamp: NaiveDateTime,
    pub task: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningOperation {
    Request(ToolingChatRequest),
    Response(ToolingChatResponse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningEvent {
    Add(OperationInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningAction {
    Add(OperationInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningInfo {
    pub model: String,
    pub records: Vec<ReasoningRecord>,
}

impl ReasoningInfo {
    fn new(model: String) -> Self {
        Self {
            model,
            records: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningRecord {
    pub timestamp: NaiveDateTime,
}

/*
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
pub struct ReasoningId {
    id: Uuid,
}

impl ReasoningId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningRecord {
    pub task: String,
}
*/
