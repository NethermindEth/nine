use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_core::tracers::tools::Tools;
use yew::{html, Html};

pub type ToolsListWidget = SubWidget<ToolsListComponent>;

pub struct ToolsListComponent {}

impl SubComponent for ToolsListComponent {
    type Projection = single::Flow<Tools>;
    type Message = ();

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: single::State<Tools>, ctx: &SubContext<Self>) -> Option<Html> {
        // TODO: Use custom classes
        let pairs = state.tools_list.iter();
        Some(html! {
            <div class="widget-session-control">
                <div class="widget-session-control-header">
                    <div class="widget-session-control-header-title">
                        { "Tools" }
                    </div>
                </div>
                <div class="widget-session-control-list">
                    { for pairs.map(|(k, v)| self.render_item(k, v, ctx)) }
                </div>
            </div>
        })
    }
}

impl ToolsListComponent {
    fn render_item(&self, tool: &str, desc: &str, ctx: &SubContext<Self>) -> Html {
        html! {
            <div class="widget-session-control-list-item">
                { tool }
            </div>
        }
    }
}
