use super::flow::Dashboard;
use crate::render::{single, SubComponent, SubContext, SubWidget};
use crate::widgets;
use n9_control_chat::Chat;
use n9_control_session::SessionControl;
use ui9_dui::tracers::job::Job;
use ui9_dui::{FqnLink, Unified};
use ui9_net::tracers::peer::Peer;
use ui9_net::PeerId;
use yew::{html, Html};

pub type DashboardWidget = SubWidget<DashboardComponent>;

pub struct DashboardComponent {}

impl SubComponent for DashboardComponent {
    type Projection = single::Flow<Dashboard>;
    type Message = ();

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: single::State<Dashboard>, _ctx: &SubContext<Self>) -> Option<Html> {
        let app_content = {
            if let Some(active_peer) = state.active_peer {
                let peer = active_peer.to_string();
                let chat_link = FqnLink::remote(Chat::fqn(), active_peer);
                let jobs_link = FqnLink::remote(Job::fqn(), active_peer);
                let chat_interaction: FqnLink = Dashboard::fqn().into();
                let session_control = {
                    let first = FqnLink::remote(SessionControl::fqn(), active_peer);
                    let second: FqnLink = Dashboard::fqn().into();
                    html! {
                        <widgets::SessionControl {first} {second} />
                    }
                };
                html! {
                    <div class="app-content">
                        <div class="app-content-left">
                            { session_control }
                        </div>
                        <div class="app-content-center">
                            <widgets::ChatInteraction link={chat_interaction} />
                            /*
                            <div>{ "Chat of the peer: " }{ peer }</div>
                            <widgets::Chat link={chat_link} />
                            <widgets::Jobs link={jobs_link} />
                            */
                        </div>
                        <div class="app-content-right">
                        </div>
                    </div>
                }
            } else {
                let first: FqnLink = Peer::fqn().into();
                let second: FqnLink = Dashboard::fqn().into();
                html! {
                    <div class="app-content">
                        <div class="app-content-left">
                        </div>
                        <div class="app-content-center">
                            <widgets::Peers {first} {second} />
                        </div>
                        <div class="app-content-right">
                        </div>
                    </div>
                }
            }
        };
        html! {
            <div class="app theme-light">
                <div class="app-header">
                    <div class="loader">
                        <div class="loader-container">
                            <img src="static/logos/SVG/Nine_Icon.svg" />
                            <div class="loader-overlay"></div>
                        </div>
                    </div>
                    <div class="app-header-title">{ "N9 Dashboard" }</div>
                </div>

                { app_content }
            </div>
        }
        .into()
    }
}
