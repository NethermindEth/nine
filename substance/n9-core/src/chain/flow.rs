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
    pub fn request(&self, req: ToolingChatRequest) {
        let action = ReasoningEvent::Request(req);
        self.action(action);
    }

    pub fn response(&self, res: ToolingChatResponse) {
        let action = ReasoningEvent::Response(res);
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
    /*
    pub fn event(&self, event: ReasoningEvent) {
        self.event(event);
    }
    */
}

impl Publisher for ReasoningFlow {
    type Driver = ReasoningPub;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningFlow {
    pub actions: Vec<ReasoningEvent>,
}

impl Default for ReasoningFlow {
    fn default() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
}

impl Flow for ReasoningFlow {
    type Event = ReasoningEvent;
    type Action = ReasoningEvent;

    fn apply(&mut self, event: Self::Event) {
        self.actions.push(event);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningEvent {
    Request(ToolingChatRequest),
    Response(ToolingChatResponse),
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
