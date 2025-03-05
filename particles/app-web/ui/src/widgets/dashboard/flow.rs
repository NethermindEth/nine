use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;
use ui9_dui::{Flow, Listener, Publisher, Subscriber, Tracer, Unified};

#[derive(Deref, DerefMut, From, Into)]
pub struct DashboardSub {
    listener: Listener<Dashboard>,
}

impl Subscriber for Dashboard {
    type Driver = DashboardSub;
}

impl DashboardSub {}

#[derive(Deref, DerefMut, From, Into)]
pub struct DashboardPub {
    tracer: Tracer<Dashboard>,
}

impl Publisher for Dashboard {
    type Driver = DashboardPub;
}

impl DashboardPub {}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Dashboard {}

impl Unified for Dashboard {
    fn fqn() -> Fqn {
        Fqn::root("@web-app")
    }
}

impl Flow for Dashboard {
    type Event = DashboardEvent;
    type Action = DashboardAction;

    fn apply(&mut self, event: Self::Event) {}
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DashboardEvent {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DashboardAction {}
