use crate::render::{SubWidget, SubComponent};
use ui9_dui::State;
use ui9_dui::tracers::event::Event;
use yew::{Html, html};

pub type PeersList = SubWidget<Peers>;

pub struct Peers {
}

impl SubComponent for Peers {
    type Flow = Event;

    fn create() -> Self {
        Self {
        }
    }

    fn render(&self, state: &State<Self::Flow>) -> Option<Html> {
        let typ = std::any::type_name::<Self::Flow>();
        Some(html! {
            <div>{ format!("Loaded: {typ}") }</div>
        })
    }
}
