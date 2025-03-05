use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use ui9::names::Fqn;
use ui9_dui::{Flow, Listener, Publisher, Subscriber, Tracer, Unified};

#[derive(Deref, DerefMut, From, Into)]
pub struct WebAppSub {
    listener: Listener<WebApp>,
}

impl Subscriber for WebApp {
    type Driver = WebAppSub;
}

impl WebAppSub {}

#[derive(Deref, DerefMut, From, Into)]
pub struct WebAppPub {
    tracer: Tracer<WebApp>,
}

impl Publisher for WebApp {
    type Driver = WebAppPub;
}

impl WebAppPub {}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct WebApp {}

impl Unified for WebApp {
    fn fqn() -> Fqn {
        Fqn::root("@web-app")
    }
}

impl Flow for WebApp {
    type Event = WebAppEvent;
    type Action = WebAppAction;

    fn apply(&mut self, event: Self::Event) {}
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum WebAppEvent {}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum WebAppAction {}
