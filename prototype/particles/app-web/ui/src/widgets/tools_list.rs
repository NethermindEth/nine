use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_core::tracers::tools::Tools;
use std::collections::BTreeSet;
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
            <div class="widget-tools-list">
                <div class="widget-tools-list-header">
                    <div class="widget-tools-list-header-title">
                        { "Toolkits" }
                    </div>
                    <div class="widget-tools-list-header-new">
                        { "Edit" }
                    </div>
                </div>
                { for pairs.map(|(k, v)| self.render_toolkit(k, v, ctx)) }
            </div>
        })
    }
}

impl ToolsListComponent {
    fn render_toolkit(
        &self,
        toolkit: &str,
        actions: &BTreeSet<String>,
        ctx: &SubContext<Self>,
    ) -> Html {
        html! {
            <div class="widget-tools-list-toolkit">
                <div class="widget-tools-list-toolkit-name">{ toolkit }</div>
                <div>
                    { for actions.iter().map(|item| self.render_item(item, ctx)) }
                </div>
            </div>
        }
    }

    fn render_item(&self, action: &str, ctx: &SubContext<Self>) -> Html {
        html! {
            <div class="widget-tools-list-toolkit-action">
                { action }
            </div>
        }
    }
}
