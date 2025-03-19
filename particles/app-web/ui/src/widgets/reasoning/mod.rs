use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_core::chain::ReasoningFlow;
use std::mem::swap;
use ui9_markdown::MarkdownRender;
use yew::{html, Html, InputEvent, TargetCast};

pub type ReasoningSummaryWidget = SubWidget<ReasoningSummaryComponent>;

pub struct ReasoningSummaryComponent {}

#[derive(Clone)]
pub enum Msg {}

impl SubComponent for ReasoningSummaryComponent {
    type Projection = single::Flow<ReasoningFlow>;
    type Message = Msg;

    fn create() -> Self {
        Self {}
    }

    fn update(
        &mut self,
        msg: Self::Message,
        pro: &mut Self::Projection,
        _ctx: &SubContext<Self>,
    ) -> bool {
        true
    }

    fn render(&self, state: single::State<ReasoningFlow>, ctx: &SubContext<Self>) -> Option<Html> {
        let operation = {
            if state.completed {
                None
            } else {
                let operation = state.operations.last().map(|info| &info.task);
                html! {
                    <div>{ operation }</div>
                }
                .into()
            }
        };
        html! {
            <div class="widget-reasoning-summary">
                <div class="widget-reasoning-summary-requests">{ state.stat.requests }</div>
                <div class="widget-reasoning-summary-calls">{ state.stat.calls }</div>
                <div>{ operation }</div>
            </div>
        }
        .into()
    }
}
