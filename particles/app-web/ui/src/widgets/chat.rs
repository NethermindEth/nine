use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_control_session::{ChatControl, Message, Role};
use ui9_markdown::MarkdownRender;
use yew::{html, Html};

pub type ChatWidget = SubWidget<ChatComponent>;

pub struct ChatComponent {
    render: MarkdownRender,
}

impl SubComponent for ChatComponent {
    type Projection = single::Flow<ChatControl>;
    type Message = ();

    fn create() -> Self {
        Self {
            render: MarkdownRender::new(),
        }
    }

    fn render(&self, state: single::State<ChatControl>, _ctx: &SubContext<Self>) -> Option<Html> {
        let body = {
            if state.is_empty() {
                html! {
                    <div class="widget-chat-empty">
                        <textarea />
                    </div>
                }
            } else {
                html! {
                    <div class="widget-chat-filled">
                        <div class="widget-chat-dialog">
                            { for state.messages.iter().map(|msg| self.render(msg)) }
                        </div>
                    </div>
                }
            }
        };
        html! {
            <div class="widget-chat">
                { body }
            </div>
        }
        .into()
    }
}

impl ChatComponent {
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
