use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use ui9::names::Fqn;
use ui9_dui::{Flow, Listener, Publisher, Subscriber, Tracer, Unified};

#[derive(Deref, DerefMut, From, Into)]
pub struct SessionControlSub {
    listener: Listener<SessionControl>,
}

impl Subscriber for SessionControl {
    type Driver = SessionControlSub;
}

impl SessionControlSub {
    pub fn create(&mut self, key: SessionKey) {
        let event = SessionControlAction::Create { key };
        self.listener.action(event);
    }
}

#[derive(Deref, DerefMut, From, Into)]
pub struct SessionControlPub {
    tracer: Tracer<SessionControl>,
}

impl Publisher for SessionControl {
    type Driver = SessionControlPub;
}

impl SessionControlPub {
    /*
    pub fn add(&mut self, content: String, role: Role) {
        let message = Message { content, role };
        let event = SessionControlEvent::Add { message };
        self.tracer.event(event);
    }
    */
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct SessionControl {
    pub active_sessions: BTreeMap<SessionKey, SessionInfo>,
}

impl Unified for SessionControl {
    fn fqn() -> Fqn {
        Fqn::root("@control-session")
    }
}

impl Flow for SessionControl {
    type Event = SessionControlEvent;
    type Action = SessionControlAction;

    fn apply(&mut self, event: Self::Event) {
        match event {
            SessionControlEvent::Add { key, fqn } => {
                let info = SessionInfo { fqn };
                self.active_sessions.insert(key, info);
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SessionControlEvent {
    Add {
        key: SessionKey,
        // TODO: Use `RemoteFqn`?
        fqn: Fqn,
    },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SessionControlAction {
    Create { key: SessionKey },
}

pub type SessionKey = String;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SessionInfo {
    pub fqn: Fqn,
}
