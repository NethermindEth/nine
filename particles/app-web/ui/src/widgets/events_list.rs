use crate::render::{SingleFlow, SingleState, SubComponent, SubWidget};
use ui9_dui::tracers::event::Event;
use yew::{html, Html};

pub type EventsList = SubWidget<Events>;

pub struct Events {}

impl SubComponent for Events {
    type Projection = SingleFlow<Event>;

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: SingleState<Event>) -> Option<Html> {
        let typ = std::any::type_name::<Event>();
        Some(html! {
            <div>{ format!("Loaded: {typ}") }</div>
        })
    }
}
