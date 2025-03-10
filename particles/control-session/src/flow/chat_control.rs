use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;
use ui9_dui::{Flow, Listener, Publisher, Subscriber, Tracer, Unified};

#[derive(Deref, DerefMut, From, Into)]
pub struct ChatControlSub {
    listener: Listener<ChatControl>,
}

impl Subscriber for ChatControl {
    type Driver = ChatControlSub;
}

impl ChatControlSub {
    /*
    pub fn create(&mut self, key: ChatKey) {
        let event = ChatControlAction::Create { key };
        self.listener.action(event);
    }
    */
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
pub struct ChatControl {}

impl Flow for ChatControl {
    type Event = ChatControlEvent;
    type Action = ChatControlAction;

    fn apply(&mut self, event: Self::Event) {}
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatControlEvent {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ChatControlAction {}
