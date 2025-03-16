use crate::render::{single, SubComponent, SubContext, SubWidget};
use crate::widgets;
use n9_control_session::{ChatControl, ChatTurn, Message, Role};
use std::mem::swap;
use ui9_dui::FqnLink;
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
    Send,
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
        pro: &mut Self::Projection,
        _ctx: &SubContext<Self>,
    ) -> bool {
        match msg {
            Msg::UpdateText(text) => {
                self.text = text;
            }
            Msg::Send => {
                let mut text = String::new();
                swap(&mut text, &mut self.text);
                pro.prompt(text);
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
                            <div class="widget-chat-dialog-viewport">
                                { for state.items.iter().map(|item| self.render_item(item)) }
                            </div>
                        </div>
                        { self.render_input(ctx) }
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

fn text(e: InputEvent) -> String {
    let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
    input.value()
}

impl ChatComponent {
    fn render_input(&self, ctx: &SubContext<Self>) -> Html {
        let oninput = ctx.callback(|e: InputEvent| Msg::UpdateText(text(e)));
        let send = ctx.event(Msg::Send);
        let value = self.text.clone();
        html! {
            <div class="widget-chat-input">
                <textarea {oninput} {value} />
                <div class="widget-chat-input-controls">
                    <div class="widget-chat-input-controls-left">
                    </div>
                    <div class="widget-chat-input-controls-right">
                        <div onclick={send} class="widget-chat-input-controls-send">{ "Send" }</div>
                    </div>
                </div>
            </div>
        }
    }

    fn render_item(&self, item: &ChatTurn) -> Html {
        let tracer = {
            if let Some(tracer) = &item.tracer {
                let link = FqnLink::from(tracer.clone());
                Some(html! {
                    <div class="widget-chat-reasoning">
                        <widgets::ReasoningSummary {link} />
                    </div>
                })
            } else {
                None
            }
        };
        html! {
            <div class="widget-chat-turn">
                <div class="widget-chat-message">
                    <div class="widget-chat-request">
                        { item.request.as_ref().map(|req| &req.content) }
                    </div>
                </div>
                { tracer }
                <div class="widget-chat-message">
                    <div class="widget-chat-response">
                        { item.request.as_ref().map(|req| &req.content) }
                    </div>
                </div>
            </div>
        }
    }

    /*
    fn render_message(&self, msg: &Message) -> Html {
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
    */
}
