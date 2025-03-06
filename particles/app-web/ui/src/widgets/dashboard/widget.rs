use super::flow::Dashboard;
use crate::render::{single, FqnLink, SubComponent, SubContext, SubWidget};
use crate::widgets;
use n9_control_chat::Chat;
use ui9_dui::tracers::job::Job;
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

    fn render(&self, state: single::State<Dashboard>, _ctx: &SubContext<Self>) -> Option<Html> {
        let chat_or_peers;
        if let Some(active_peer) = state.active_peer {
            let peer = active_peer.to_string();
            let chat_link = FqnLink::remote(Chat::fqn(), active_peer);
            let jobs_link = FqnLink::remote(Job::fqn(), active_peer);
            chat_or_peers = html! {
                <div>
                    <div>{ "Chat of the peer: " }{ peer }</div>
                    <widgets::Chat link={chat_link} />
                    <widgets::Jobs link={jobs_link} />
                </div>
            };
        } else {
            let first: FqnLink = Peer::fqn().into();
            let second: FqnLink = Dashboard::fqn().into();
            chat_or_peers = html! {
                <widgets::Peers {first} {second} />
            };
        }
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
                    { chat_or_peers }
                </div>
            </div>
        }
        .into()
    }
}
