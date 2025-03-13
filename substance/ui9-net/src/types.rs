use serde::{Deserialize, Serialize};
use ui9::names::Fqn;
use libp2p::PeerId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FqnLink {
    pub fqn: Fqn,
    pub peer: Option<PeerId>,
}

impl From<Fqn> for FqnLink {
    fn from(fqn: Fqn) -> Self {
        Self { fqn, peer: None }
    }
}

impl FqnLink {
    pub fn remote(fqn: Fqn, peer: PeerId) -> Self {
        Self {
            fqn,
            peer: Some(peer),
        }
    }
}
