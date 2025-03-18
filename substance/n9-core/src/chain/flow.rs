use crate::router::types::{ToolCall, ToolingChatRequest, ToolingChatResponse};
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
    pub fn request(&self, request: ToolingChatRequest) {
        let operation = Operation::Request(request);
        let action = ReasoningAction::Operation(operation);
        self.action(action);
    }

    pub fn response(&self, response: ToolingChatResponse) {
        let operation = Operation::Response(response);
        let action = ReasoningAction::Operation(operation);
        self.action(action);
    }

    pub fn tool_call(&self, call: ToolCall) {
        let operation = Operation::ToolCall(call);
        let action = ReasoningAction::Operation(operation);
        self.action(action);
    }

    // TODO: Take out to the separate service

    pub fn show(&self, id: OperationId) {
        let action = ReasoningAction::Show(id);
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

    pub fn show(&self, operation: Option<OperationDetails>) {
        let event = ReasoningEvent::Show(operation);
        self.event(event);
    }
}

impl Publisher for ReasoningFlow {
    type Driver = ReasoningPub;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningFlow {
    pub operations: Vec<OperationInfo>,
    pub operation: Option<OperationDetails>,
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
        match event {
            ReasoningEvent::Add(operation) => {
                self.operations.push(operation);
            }
            ReasoningEvent::Show(operation) => {
                self.operation = operation;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationInfo {
    pub id: OperationId,
    pub timestamp: NaiveDateTime,
    pub task: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDetails {
    pub id: OperationId,
    pub operation: Operation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningEvent {
    Add(OperationInfo),
    Show(Option<OperationDetails>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningAction {
    Show(Uuid),
    Operation(Operation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Request(ToolingChatRequest),
    Response(ToolingChatResponse),
    ToolCall(ToolCall),
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

pub type OperationId = Uuid;
