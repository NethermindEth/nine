use crate::router::types::{ToolCall, ToolResult, ToolingChatRequest, ToolingChatResponse};
use chrono::NaiveDateTime;
use crb::core::uuid::Uuid;
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use ui9_dui::flow::Flow;
use ui9_dui::publisher::{Publisher, Tracer};
use ui9_dui::subscriber::{Listener, Subscriber};

#[derive(Deref, DerefMut, From, Into)]
pub struct UnrollerSub {
    listener: Listener<UnrollerFlow>,
}

impl UnrollerSub {
    pub fn request(&self, request: ToolingChatRequest) {
        let operation = Operation::Request(request);
        let action = UnrollerAction::Operation(operation);
        self.action(action);
    }

    pub fn response(&self, response: ToolingChatResponse) {
        let operation = Operation::Response(response);
        let action = UnrollerAction::Operation(operation);
        self.action(action);
    }

    pub fn tool_call(&self, call: ToolCall) {
        let operation = Operation::ToolCall(call);
        let action = UnrollerAction::Operation(operation);
        self.action(action);
    }

    pub fn tool_call_result(&self, response: ToolResult) {
        let operation = Operation::ToolResult(response);
        let action = UnrollerAction::Operation(operation);
        self.action(action);
    }

    pub fn done(&self) {
        let action = UnrollerAction::Done;
        self.action(action);
    }

    // TODO: Take out to the separate service

    pub fn show(&self, id: OperationId) {
        let action = UnrollerAction::Show(id);
        self.action(action);
    }
}

impl Subscriber for UnrollerFlow {
    type Driver = UnrollerSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct UnrollerPub {
    tracer: Tracer<UnrollerFlow>,
}

impl UnrollerPub {
    pub fn operation(&self, info: OperationInfo) {
        let event = UnrollerEvent::Add(info);
        self.event(event);
    }

    pub fn complete(&self) {
        let event = UnrollerEvent::Complete;
        self.event(event);
    }

    pub fn show(&self, operation: Option<OperationDetails>) {
        let event = UnrollerEvent::Show(operation);
        self.event(event);
    }
}

impl Publisher for UnrollerFlow {
    type Driver = UnrollerPub;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnrollerStat {
    pub calls: u32,
    pub requests: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnrollerFlow {
    pub operations: Vec<OperationInfo>,
    pub operation: Option<OperationDetails>,
    pub stat: UnrollerStat,
    // TODO: Add Reason (or error)
    pub completed: bool,
}

impl Default for UnrollerFlow {
    fn default() -> Self {
        Self {
            operations: Vec::new(),
            operation: None,
            stat: UnrollerStat::default(),
            completed: false,
        }
    }
}

impl Flow for UnrollerFlow {
    type Event = UnrollerEvent;
    type Action = UnrollerAction;

    fn apply(&mut self, event: Self::Event) {
        match event {
            UnrollerEvent::Add(operation) => {
                match &operation.op_type {
                    OperationType::Request => {
                        self.stat.requests += 1;
                    }
                    OperationType::ToolCall => {
                        self.stat.calls += 1;
                    }
                    _ => {}
                }
                self.operations.push(operation);
            }
            UnrollerEvent::Complete => {
                self.completed = true;
            }
            UnrollerEvent::Show(operation) => {
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
pub enum UnrollerEvent {
    Add(OperationInfo),
    Complete,
    Show(Option<OperationDetails>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnrollerAction {
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
            Self::Request(_) => OperationType::Request,
            Self::Response(_) => OperationType::Response,
            Self::ToolCall(_) => OperationType::ToolCall,
            Self::ToolResult(_) => OperationType::ToolResult,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnrollerInfo {
    pub model: String,
    pub records: Vec<UnrollerRecord>,
}

impl UnrollerInfo {
    fn new(model: String) -> Self {
        Self {
            model,
            records: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnrollerRecord {
    pub timestamp: NaiveDateTime,
}

pub type OperationId = Uuid;
