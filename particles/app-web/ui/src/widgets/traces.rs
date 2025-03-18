use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_core::chain::{OperationId, OperationInfo, ReasoningFlow};
use std::mem::swap;
use ui9_markdown::MarkdownRender;
use yew::{html, Html, InputEvent, TargetCast};

pub type TracesWidget = SubWidget<TracesComponent>;

pub struct TracesComponent {}

#[derive(Clone)]
pub enum Msg {
    Load(OperationId),
}

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
        match msg {
            Msg::Load(id) => {
                // TODO: Rename to `load`
                pro.show(id);
            }
        }
        true
    }

    fn render(&self, state: single::State<ReasoningFlow>, ctx: &SubContext<Self>) -> Option<Html> {
        let details = state.operation.as_ref().map(|operation| {
            html! {
                { format!("{operation:?}") }
            }
        });
        html! {
            <div class="widget-traces">
                <div class="widget-traces-list">
                    { for state.operations.iter().map(|op| self.render_operation(op, ctx)) }
                </div>
                <div class="widget-traces-details">
                    { details }
                </div>
            </div>
        }
        .into()
    }
}

impl TracesComponent {
    fn render_operation(&self, op: &OperationInfo, ctx: &SubContext<Self>) -> Html {
        let onclick = ctx.event(Msg::Load(op.id));
        html! {
            <div {onclick} class="widget-traces-list-item">
                { &op.task }
            </div>
        }
    }
}
