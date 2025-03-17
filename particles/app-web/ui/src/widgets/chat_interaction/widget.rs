use crate::render::{single, SubComponent, SubContext, SubWidget};
use crate::widgets;
use crate::widgets::dashboard::Dashboard;
use ui9_dui::FqnLink;
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
        if let Some(link) = state.active_traces.clone() {
            let link = FqnLink::from(link);
            html! {
                <widgets::Traces {link} />
            }
            .into()
        } else if let Some(link) = state.active_chat.clone() {
            html! {
                <widgets::Chat {link} />
            }
            .into()
        } else {
            html! {
                <div class="widget-chat">
                </div>
            }
            .into()
        }
    }
}
