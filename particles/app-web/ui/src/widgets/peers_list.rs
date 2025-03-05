use crate::render::{SingleFlow, SingleState, SubComponent, SubWidget};
use ui9_net::tracers::peer::{Peer, PeerId};
use yew::{html, Html};

pub type PeersList = SubWidget<Peers>;

pub struct Peers {}

impl SubComponent for Peers {
    type Projection = SingleFlow<Peer>;

    fn create() -> Self {
        Self {}
    }

    fn render(&self, state: SingleState<Peer>) -> Option<Html> {
        let typ = std::any::type_name::<Peer>();
        let list = {
            if state.peers.is_empty() {
                html! {
                    <div class="component-peers-peer">{ format!("No peers yet :)") }</div>
                }
            } else {
                html! {
                    <div>
                        { for state.peers.keys().map(|peer| self.render_peer(peer)) }
                    </div>
                }
            }
        };
        Some(html! {
            <div class="component-peers">
                <div class="component-peers-header">{ "Peers" }</div>
                { list }
            </div>
        })
    }
}

impl Peers {
    fn render_peer(&self, peer: &PeerId) -> Html {
        html! {
            <div class="component-peers-peer">{ peer.to_string() }</div>
        }
    }
}
