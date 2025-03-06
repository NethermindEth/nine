use crate::render::{single, SubComponent, SubContext, SubWidget};
use ui9_dui::tracers::event::Event;
use yew::{html, Html};

pub type EventsWidget = SubWidget<EventsComponent>;

pub struct EventsComponent {}

impl SubComponent for EventsComponent {
    type Projection = single::Flow<Event>;
    type Message = ();

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: single::State<Event>, _ctx: &SubContext<Self>) -> Option<Html> {
        let typ = std::any::type_name::<Event>();
        Some(html! {
            <div>{ format!("Loaded: {typ}") }</div>
        })
    }
}
