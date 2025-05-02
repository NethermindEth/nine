use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_core::unroller::UnrollerFlow;
use yew::{html, Html};

pub type UnrollerSummaryWidget = SubWidget<UnrollerSummaryComponent>;

pub struct UnrollerSummaryComponent {}

#[derive(Clone)]
pub enum Msg {}

impl SubComponent for UnrollerSummaryComponent {
    type Projection = single::Flow<UnrollerFlow>;
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

    fn render(&self, state: single::State<UnrollerFlow>, ctx: &SubContext<Self>) -> Option<Html> {
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
