use derive_more::Deref;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub type ElementId = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtomId {
    pub peer_id: PeerId,
    pub path: Vec<ElementId>,
}

#[derive(Deref)]
pub struct TypedAtomId<A> {
    #[deref]
    pub atom_id: AtomId,
    _type: PhantomData<A>,
}
