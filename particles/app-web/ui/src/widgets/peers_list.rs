use crate::render::{SubComponent, SubWidget};
use ui9_dui::State;
use yew::{html, Html};
use ui9_net::tracers::peer::Peer;

pub type PeersList = SubWidget<Peers>;

pub struct Peers {}

impl SubComponent for Peers {
    type Flow = Peer;

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: &State<Self::Flow>) -> Option<Html> {
        let typ = std::any::type_name::<Self::Flow>();
        Some(html! {
            <div>{ format!("Loaded: {typ}") }</div>
        })
    }
}
