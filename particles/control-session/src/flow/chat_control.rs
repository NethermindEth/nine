use derive_more::{Deref, DerefMut, From, Into};
use n9_core::chain::ReasoningFlow;
use serde::{Deserialize, Serialize};
use ui9_dui::{Flow, Link, Listener, Publisher, Subscriber, Tracer};

#[derive(Deref, DerefMut, From, Into)]
pub struct ChatControlSub {
    listener: Listener<ChatControl>,
}

impl Subscriber for ChatControl {
    type Driver = ChatControlSub;
}

impl ChatControlSub {
    pub fn prompt(&self, prompt: String) {
        let event = ChatControlAction::Prompt { prompt };
        self.listener.action(event);
    }
}

#[derive(Deref, DerefMut, From, Into)]
pub struct ChatControlPub {
    tracer: Tracer<ChatControl>,
}

impl Publisher for ChatControl {
    type Driver = ChatControlPub;
}

impl ChatControlPub {
    pub fn add(&mut self, content: String, role: Role) {
        let message = Message { content, role };
        let item = ChatItem::Message(message);
        let event = ChatControlEvent::AddItem { item };
        self.tracer.event(event);
    }

    pub fn start_thinking(&mut self, link: Link<ReasoningFlow>) {
        let item = ChatItem::Tracer(link);
        let event = ChatControlEvent::AddItem { item };
        self.tracer.event(event);
    }

    pub fn stop_thinking(&mut self) {}
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct ChatControl {
    pub items: Vec<ChatItem>,
}

impl ChatControl {
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Flow for ChatControl {
    type Event = ChatControlEvent;
    type Action = ChatControlAction;

    fn apply(&mut self, event: Self::Event) {
        match event {
            ChatControlEvent::AddItem { item } => {
                self.items.push(item);
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatTurn {
    pub request: Option<ChatRequest>,
    pub response: Option<ChatResponse>,
    pub tracer: Option<Link<ReasoningFlow>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    pub content: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatResponse {
    pub content: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatItem {
    Message(Message),
    Tracer(Link<ReasoningFlow>),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatControlEvent {
    AddItem { item: ChatItem },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatControlAction {
    Prompt { prompt: String },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Role {
    Request,
    Response,
}
