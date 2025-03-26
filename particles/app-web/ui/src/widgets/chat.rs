use crate::render::{double, SubComponent, SubContext, SubWidget};
use crate::widgets;
use crate::widgets::dashboard::Dashboard;
use n9_control_session::{ChatControl, ChatRequest, ChatResponse, ChatTurn};
use n9_core::unroller::UnrollerFlow;
use std::mem::swap;
use ui9_dui::{FqnLink, Link};
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
    OpenTraces(Link<UnrollerFlow>),
}

impl SubComponent for ChatComponent {
    type Projection = double::Flow<ChatControl, Dashboard>;
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
                pro.first.prompt(text);
            }
            Msg::OpenTraces(link) => {
                pro.second.open_traces(Some(link));
            }
        }
        true
    }

    fn render(
        &self,
        state: double::State<ChatControl, Dashboard>,
        ctx: &SubContext<Self>,
    ) -> Option<Html> {
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
                                { for state.items.iter().map(|item| self.render_item(item, ctx)) }
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

    fn render_item(&self, item: &ChatTurn, ctx: &SubContext<Self>) -> Html {
        // TODO: Use a better indicator + errors reporting
        let tracer = {
            if let Some(tracer) = &item.tracer {
                let onclick = ctx.event(Msg::OpenTraces(tracer.clone()));
                let link = FqnLink::from(tracer.clone());
                Some(html! {
                    <div class="widget-chat-reasoning">
                        <widgets::UnrollerSummary {link} />
                        <div {onclick} class="widget-chat-reasoning-open">{ "Open traces" }</div>
                    </div>
                })
            } else {
                None
            }
        };
        html! {
            <div class="widget-chat-turn">
                { item.request.as_ref().map(|req| self.render_request(req)) }
                { tracer }
                { item.response.as_ref().map(|res| self.render_response(res)) }
            </div>
        }
    }

    fn render_request(&self, req: &ChatRequest) -> Html {
        let content = self.render.block(&req.content);
        html! {
            <div class="widget-chat-message">
                <div class="widget-chat-request">
                    { content }
                </div>
            </div>
        }
    }

    fn render_response(&self, res: &ChatResponse) -> Html {
        let content = self.render.block(&res.content);
        html! {
            <div class="widget-chat-message">
                <div class="widget-chat-response">
                    { content }
                </div>
            </div>
        }
    }
}
