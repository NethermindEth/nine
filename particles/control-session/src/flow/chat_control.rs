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
    pub fn new_turn(&mut self) {
        // TODO: Add prompt here
        let event = ChatControlEvent::NewTurn;
        self.tracer.event(event);
    }

    pub fn add(&mut self, content: String, role: Role) {
        let message = Message { content, role };
        let event = ChatControlEvent::Message(message);
        self.tracer.event(event);
    }

    pub fn start_thinking(&mut self, link: Link<ReasoningFlow>) {
        let event = ChatControlEvent::Tracer(link);
        self.tracer.event(event);
    }

    pub fn stop_thinking(&mut self) {}
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct ChatControl {
    pub items: Vec<ChatTurn>,
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
            ChatControlEvent::NewTurn => {
                let turn = ChatTurn::default();
                self.items.push(turn);
            }
            ChatControlEvent::Tracer(link) => {
                if let Some(last) = self.items.last_mut() {
                    last.tracer = Some(link);
                }
            }
            ChatControlEvent::Message(message) => {
                if let Some(last) = self.items.last_mut() {
                    match message.role {
                        Role::Request => {
                            let request = ChatRequest {
                                content: message.content,
                            };
                            last.request = Some(request);
                        }
                        Role::Response => {
                            let response = ChatResponse {
                                content: message.content,
                            };
                            last.response = Some(response);
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
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
pub enum ChatControlEvent {
    NewTurn,
    Tracer(Link<ReasoningFlow>),
    Message(Message),
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
