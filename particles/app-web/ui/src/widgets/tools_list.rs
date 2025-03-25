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
                        { "Tools" }
                    </div>
                    <div class="widget-tools-list-header-new">
                        { "Edit" }
                    </div>
                </div>
                <div class="widget-tools-list-list">
                    { for pairs.map(|(k, v)| self.render_toolkit(k, v, ctx)) }
                </div>
            </div>
        })
    }
}

impl ToolsListComponent {
    fn render_toolkit(
        &self,
        toolkit: &str,
        skills: &BTreeSet<String>,
        ctx: &SubContext<Self>,
    ) -> Html {
        html! {
            <div class="widget-tools-list-list-item">
                { toolkit }
                <div>
                    { for skills.iter().map(|item| self.render_item(item, ctx)) }
                </div>
            </div>
        }
    }

    fn render_item(&self, skill: &str, ctx: &SubContext<Self>) -> Html {
        html! {
            <div class="widget-tools-list-list-item">
                { skill }
            </div>
        }
    }
}
