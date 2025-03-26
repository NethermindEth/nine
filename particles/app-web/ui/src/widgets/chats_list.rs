use crate::render::{single, SubComponent, SubContext, SubWidget};
use ui9_dui::tracers::event::Event;
use yew::{html, Html};

pub type ChatsListWidget = SubWidget<ChatsListComponent>;

pub struct ChatsListComponent {}

impl SubComponent for ChatsListComponent {
    type Projection = single::Flow<Event>;
    type Message = ();

    fn create() -> Self {
        Self {}
    }

    fn render(&self, _state: single::State<Event>, _ctx: &SubContext<Self>) -> Option<Html> {
        let typ = std::any::type_name::<Event>();
        Some(html! {
            <div>{ format!("Loaded: {typ}") }</div>
        })
    }
}
