use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use ui9_dui::{Flow, Listener, Publisher, Subscriber, Tracer};

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
    /*
    pub fn add(&mut self, content: String, role: Role) {
        let message = Message { content, role };
        let event = ChatControlEvent::Add { message };
        self.tracer.event(event);
    }
    */
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct ChatControl {
    pub messages: Vec<Message>,
}

impl ChatControl {
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl Flow for ChatControl {
    type Event = ChatControlEvent;
    type Action = ChatControlAction;

    fn apply(&mut self, event: Self::Event) {
        match event {
            ChatControlEvent::Add { message } => {
                self.messages.push(message);
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatControlEvent {
    Add { message: Message },
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
