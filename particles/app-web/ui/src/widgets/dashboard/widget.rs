use super::flow::Dashboard;
use crate::render::{single, SubComponent, SubWidget};
use crate::widgets::PeersList;
use ui9_dui::Unified;
use ui9_net::tracers::peer::Peer;
use yew::{html, Html};

pub type DashboardWidget = SubWidget<DashboardComponent>;

pub struct DashboardComponent {}

impl SubComponent for DashboardComponent {
    type Projection = single::Flow<Dashboard>;
    type Message = ();

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: single::State<Dashboard>) -> Option<Html> {
        let peer = {
            if let Some(active_peer) = state.active_peer {
                html! {
                    <p>{ active_peer.to_string() }</p>
                }
            } else {
                html! {
                    <p>{ "No selected peer" }</p>
                }
            }
        };
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
                    { peer }
                    // <EventsList fqn={Event::fqn()} />
                    <PeersList first={Peer::fqn()} second={Dashboard::fqn()} />
                </div>
            </div>
        }
        .into()
    }
}
