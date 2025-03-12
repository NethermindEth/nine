use crate::render::{double, SubComponent, SubContext, SubWidget};
use crate::widgets::dashboard::Dashboard;
use ui9_net::tracers::peer::{Peer, PeerId};
use yew::{html, Html};

pub type PeersWidget = SubWidget<PeersComponent>;

pub struct PeersComponent {
    auto_connect: bool,
}

impl SubComponent for PeersComponent {
    type Projection = double::Flow<Peer, Dashboard>;
    type Message = Option<PeerId>;

    fn create() -> Self {
        Self { auto_connect: true }
    }

    fn update(
        &mut self,
        msg: Self::Message,
        pro: &mut Self::Projection,
        _ctx: &SubContext<Self>,
    ) -> bool {
        pro.second.set_peer(msg);
        true
    }

    fn discover(&mut self, state: double::State<Peer, Dashboard>, ctx: &SubContext<Self>) {
        if self.auto_connect {
            if let Some(peer_id) = state.first.peers.keys().next().cloned() {
                ctx.send(Some(peer_id));
                self.auto_connect = false;
            }
        }
    }

    fn render(
        &self,
        state: double::State<Peer, Dashboard>,
        ctx: &SubContext<Self>,
    ) -> Option<Html> {
        if self.auto_connect {
            html! {
                <div class="component-peers">
                    <div class="component-peers-peer">{ "Connecting..." }</div>
                </div>
            }
            .into()
        } else {
            let list = {
                if state.peers.is_empty() {
                    html! {
                        <div class="component-peers-peer">{ format!("No peers yet :)") }</div>
                    }
                } else {
                    html! {
                        <div>
                            { for state.peers.keys().map(|peer| self.render_peer(peer, ctx)) }
                        </div>
                    }
                }
            };
            html! {
                <div class="component-peers">
                    <div class="component-peers-header">{ "Select a peer:" }</div>
                    { list }
                </div>
            }
            .into()
        }
    }
}

impl PeersComponent {
    fn render_peer(&self, peer: &PeerId, ctx: &SubContext<Self>) -> Html {
        let onclick = ctx.event(Some(peer.clone()));
        html! {
            <div {onclick} class="component-peers-peer">{ peer.to_string() }</div>
        }
    }
}
