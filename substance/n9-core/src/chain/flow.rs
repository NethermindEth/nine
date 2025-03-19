use crate::router::types::{ToolCall, ToolResult, ToolingChatRequest, ToolingChatResponse};
use chrono::NaiveDateTime;
use crb::core::uuid::Uuid;
use derive_more::{Deref, DerefMut, Display, From, Into};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

    pub fn tool_call_result(&self, response: ToolResult) {
        let operation = Operation::ToolResult(response);
        let action = ReasoningAction::Operation(operation);
        self.action(action);
    }

    pub fn done(&self) {
        let action = ReasoningAction::Done;
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

    pub fn complete(&self) {
        let event = ReasoningEvent::Complete;
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReasoningStat {
    pub calls: u32,
    pub requests: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningFlow {
    pub operations: Vec<OperationInfo>,
    pub operation: Option<OperationDetails>,
    pub stat: ReasoningStat,
    // TODO: Add Reason (or error)
    pub completed: bool,
}

impl Default for ReasoningFlow {
    fn default() -> Self {
        Self {
            operations: Vec::new(),
            operation: None,
            stat: ReasoningStat::default(),
            completed: false,
        }
    }
}

impl Flow for ReasoningFlow {
    type Event = ReasoningEvent;
    type Action = ReasoningAction;

    fn apply(&mut self, event: Self::Event) {
        match event {
            ReasoningEvent::Add(operation) => {
                match &operation.op_type {
                    OperationType::Request => {
                        self.stat.requests += 1;
                    }
                    OperationType::ToolCall => {
                        self.stat.calls += 1;
                    }
                    _ => {
                    }
                }
                self.operations.push(operation);
            }
            ReasoningEvent::Complete => {
                self.completed = true;
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
    pub op_type: OperationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Request,
    Response,
    ToolCall,
    ToolResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationDetails {
    pub id: OperationId,
    pub operation: Operation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningEvent {
    Add(OperationInfo),
    Complete,
    Show(Option<OperationDetails>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningAction {
    Show(Uuid),
    Operation(Operation),
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Request(ToolingChatRequest),
    Response(ToolingChatResponse),
    ToolCall(ToolCall),
    ToolResult(ToolResult),
}

impl Operation {
    pub fn get_type(&self) -> OperationType {
        match self {
            Self::Request(_) => {
                OperationType::Request
            }
            Self::Response(_) => {
                OperationType::Response
            }
            Self::ToolCall(_) => {
                OperationType::ToolCall
            }
            Self::ToolResult(_) => {
                OperationType::ToolResult
            }
        }
    }
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
