use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_core::chain::{OperationInfo, ReasoningFlow};
use std::mem::swap;
use ui9_markdown::MarkdownRender;
use yew::{html, Html, InputEvent, TargetCast};

pub type TracesWidget = SubWidget<TracesComponent>;

pub struct TracesComponent {}

#[derive(Clone)]
pub enum Msg {}

impl SubComponent for TracesComponent {
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
        html! {
            <div class="widget-traces">
                <div class="widget-traces-list">
                    { for state.operations.iter().map(|op| self.render_operation(op)) }
                </div>
                <div class="widget-traces-details">
                </div>
            </div>
        }
        .into()
    }
}

impl TracesComponent {
    fn render_operation(&self, op: &OperationInfo) -> Html {
        html! {
            <div class="widget-traces-list-item">
                { &op.task }
            </div>
        }
    }
}
