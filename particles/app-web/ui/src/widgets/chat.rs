use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_control_chat::Chat;
use yew::{html, Html};

pub type ChatWidget = SubWidget<ChatComponent>;

pub struct ChatComponent {}

impl SubComponent for ChatComponent {
    type Projection = single::Flow<Chat>;
    type Message = ();

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: single::State<Chat>, _ctx: &SubContext<Self>) -> Option<Html> {
        let typ = std::any::type_name::<Chat>();
        Some(html! {
            <div>{ format!("Loaded: {typ}") }</div>
        })
    }
}
