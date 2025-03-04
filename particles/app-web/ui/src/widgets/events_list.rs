use crate::render::{SubComponent, SubWidget};
use ui9_dui::tracers::event::Event;
use ui9_dui::State;
use yew::{html, Html};

pub type EventsList = SubWidget<Events>;

pub struct Events {}

impl SubComponent for Events {
    type Flow = Event;

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: &Self::Flow) -> Option<Html> {
        let typ = std::any::type_name::<Self::Flow>();
        Some(html! {
            <div>{ format!("Loaded: {typ}") }</div>
        })
    }
}
