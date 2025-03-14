use super::Flow;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use ui9::names::Fqn;

// TODO: Implement `Serialize` and `Deserialize` that checks a type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Link<F: Flow> {
    _flow: PhantomData<F>,
    pub fqn: Fqn,
    pub peer: PeerId,
}

impl<F: Flow> Link<F> {
    pub fn new(fqn: Fqn, peer: PeerId) -> Self {
        Self {
            _flow: PhantomData,
            fqn,
            peer,
        }
    }
}

impl<F: Flow> From<Link<F>> for FqnLink {
    fn from(link: Link<F>) -> Self {
        Self {
            fqn: link.fqn,
            peer: Some(link.peer),
        }
    }
}

// TODO: Remove
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
