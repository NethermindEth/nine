use crate::render::{single, SubComponent, SubWidget};
use ui9_dui::tracers::event::Event;
use yew::{html, Html};

pub type EventsList = SubWidget<Events>;

pub struct Events {}

impl SubComponent for Events {
    type Projection = single::Flow<Event>;

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: single::State<Event>) -> Option<Html> {
        let typ = std::any::type_name::<Event>();
        Some(html! {
            <div>{ format!("Loaded: {typ}") }</div>
        })
    }
}
