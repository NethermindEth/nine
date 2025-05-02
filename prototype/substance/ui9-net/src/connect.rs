use crate::remote::RemoteExt;
use crate::MeshNode;
use ui9_dui::{Flow, Link, Listener, Sub, Subscriber};

pub trait ConnectExt<F: Flow> {
    fn connect(link: Link<F>) -> Self;
}

impl<P: Subscriber> ConnectExt<P> for Sub<P> {
    fn connect(link: Link<P>) -> Self {
        let Link { fqn, peer, .. } = link;
        let peer_id = MeshNode::peer_id();
        if peer_id == Some(peer) {
            let listener = Listener::<P>::local(fqn);
            Self::new(listener)
        } else {
            let listener = Listener::<P>::remote(peer, fqn);
            Self::new(listener)
        }
    }
}
