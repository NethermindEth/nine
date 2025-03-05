use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_control_chat::{Chat, Message, Role};
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
            <div class="widget-chat">
                { for state.messages.iter().map(|msg| self.render(msg)) }
            </div>
        })
    }
}

impl ChatComponent {
    fn render(&self, msg: &Message) -> Html {
        let class = match msg.role {
            Role::Request => "widget-chat-request",
            Role::Response => "widget-chat-response",
        };
        html! {
            <div class="widget-chat-message">
                <div {class}>{ &msg.content }</div>
            </div>
        }
    }
}
