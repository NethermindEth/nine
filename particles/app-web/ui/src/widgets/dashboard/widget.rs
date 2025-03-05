use super::flow::Dashboard;
use crate::render::{SubComponent, SubWidget};
use crate::widgets::PeersList;
use ui9_dui::Unified;
use ui9_net::tracers::peer::Peer;
use yew::{html, Html};

pub type DashboardWidget = SubWidget<DashboardComponent>;

pub struct DashboardComponent {}

impl SubComponent for DashboardComponent {
    type Flow = Dashboard;

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: &Self::Flow) -> Option<Html> {
        html! {
            <div class="app">
                <div class="app-header">
                    <div class="loader">
                        <div class="loader-container">
                            <img src="static/logo.png" />
                            <div class="loader-overlay"></div>
                        </div>
                    </div>
                    <div class="app-header-title">{ "N9 Dashboard" }</div>
                </div>

                <div class="app-content">
                    // <EventsList fqn={Event::fqn()} />
                    <PeersList fqn={Peer::fqn()} />
                </div>
            </div>
        }
        .into()
    }
}
