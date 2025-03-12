use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_control_session::{ChatControl, Message, Role};
use ui9_markdown::MarkdownRender;
use yew::{html, Html, InputEvent, TargetCast};

pub type ChatWidget = SubWidget<ChatComponent>;

pub struct ChatComponent {
    text: String,
    render: MarkdownRender,
}

#[derive(Clone)]
pub enum Msg {
    UpdateText(String),
}

impl SubComponent for ChatComponent {
    type Projection = single::Flow<ChatControl>;
    type Message = Msg;

    fn create() -> Self {
        Self {
            text: String::new(),
            render: MarkdownRender::new(),
        }
    }

    fn update(
        &mut self,
        msg: Self::Message,
        _pro: &mut Self::Projection,
        _ctx: &SubContext<Self>,
    ) -> bool {
        match msg {
            Msg::UpdateText(text) => {
                self.text = text;
            }
        }
        true
    }

    fn render(&self, state: single::State<ChatControl>, ctx: &SubContext<Self>) -> Option<Html> {
        let body = {
            if state.is_empty() {
                html! {
                    <div class="widget-chat-empty">
                        <div class="widget-chat-title">
                            { "What can I help with?" }
                        </div>
                        { self.render_input(ctx) }
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
    fn render_input(&self, ctx: &SubContext<Self>) -> Html {
        let oninput = ctx.callback(|e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            Msg::UpdateText(input.value())
        });
        let value = self.text.clone();
        html! {
            <textarea {oninput} class="widget-chat-input" {value} />
        }
    }

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
