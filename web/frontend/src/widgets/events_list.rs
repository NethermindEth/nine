use crate::render::{SubWidget, SubComponent};
use ui9_dui::tracers::event::Event;
use yew::{Html, html};

pub type EventsList = SubWidget<Events>;

pub struct Events {
}

impl SubComponent for Events {
    type Flow = Event;

    fn render(&self) -> Option<Html> {
        let typ = std::any::type_name::<Self::Flow>();
        Some(html! {
            <div>{ format!("Loaded: {typ}") }</div>
        })
    }
}
