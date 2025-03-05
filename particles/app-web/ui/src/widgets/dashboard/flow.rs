use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;
use ui9_dui::{Flow, Listener, Publisher, Subscriber, Tracer, Unified};
use ui9_net::tracers::peer::PeerId;

#[derive(Deref, DerefMut, From, Into)]
pub struct DashboardSub {
    listener: Listener<Dashboard>,
}

impl Subscriber for Dashboard {
    type Driver = DashboardSub;
}

impl DashboardSub {
    pub fn set_peer(&self, peer: Option<PeerId>) {
        let msg = DashboardMessage::SetActivePeer { peer };
        self.action(msg);
    }
}

#[derive(Deref, DerefMut, From, Into)]
pub struct DashboardPub {
    tracer: Tracer<Dashboard>,
}

impl Publisher for Dashboard {
    type Driver = DashboardPub;
}

impl DashboardPub {}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Dashboard {
    pub active_peer: Option<PeerId>,
}

impl Unified for Dashboard {
    fn fqn() -> Fqn {
        Fqn::root("@web-app")
    }
}

impl Flow for Dashboard {
    type Event = DashboardMessage;
    type Action = DashboardMessage;

    fn apply(&mut self, event: Self::Event) {
        match event {
            DashboardMessage::SetActivePeer { peer } => {
                self.active_peer = peer;
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DashboardMessage {
    SetActivePeer { peer: Option<PeerId> },
}
