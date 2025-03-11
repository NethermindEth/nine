use crate::render::{single, SubComponent, SubContext, SubWidget};
use crate::widgets::dashboard::Dashboard;
use ui9_markdown::MarkdownRender;
use yew::{html, Html};

pub type ChatInteractionWidget = SubWidget<ChatInteractionComponent>;

pub struct ChatInteractionComponent {
    render: MarkdownRender,
}

impl SubComponent for ChatInteractionComponent {
    type Projection = single::Flow<Dashboard>;
    type Message = ();

    fn create() -> Self {
        Self {
            render: MarkdownRender::new(),
        }
    }

    fn render(&self, state: single::State<Dashboard>, _ctx: &SubContext<Self>) -> Option<Html> {
        if let Some(active_peer) = &state.active_chat {
            html! {
                <div class="widget-chat">
                    // { for state.messages.iter().map(|msg| self.render(msg)) }
                </div>
            }
            .into()
        } else {
            html! {
                <div class="widget-chat">
                    { "Not connected" }
                </div>
            }
            .into()
        }
    }
}

/*
impl ChatInteractionComponent {
    fn render(&self, msg: &Message) -> Html {
        let class = match msg.role {
            Role::Request => "widget-chat-request",
            Role::Response => "widget-chat-response",
        };
        let content = self.render.block(&msg.content);
        html! {
            <div class="widget-chat-message">
                <div {class}>{ content }</div>
            </div>
        }
    }
}
*/
