use crate::render::{single, SubComponent, SubContext, SubWidget};
use ui9_dui::tracers::job::{Job, OperationId, OperationRecord};
use yew::{html, Html};

pub type JobsWidget = SubWidget<JobsComponent>;

pub struct JobsComponent {}

impl SubComponent for JobsComponent {
    type Projection = single::Flow<Job>;
    type Message = ();

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: single::State<Job>, _ctx: &SubContext<Self>) -> Option<Html> {
        if state.operations.is_empty() {
            Some(html! {
                <div>{ format!("No active jobs ʕ•́ᴥ•̀ʔ") }</div>
            })
        } else {
            Some(html! {
                <div class="widget-jobs">
                    { for state.operations.iter().map(|(id, op)| self.render_job(id, op)) }
                </div>
            })
        }
    }
}

impl JobsComponent {
    fn render_job(&self, id: &OperationId, op: &OperationRecord) -> Html {
        html! {
            <div class="widget-jobs-task">
                <div class="widget-jobs-task-id">{ id.to_string() }</div>
                <div class="widget-jobs-task-info">{ &op.task }</div>
            </div>
        }
    }
}
