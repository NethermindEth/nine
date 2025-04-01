use derive_more::Deref;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub type ElementId = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtomId {
    pub peer_id: PeerId,
    // TODO: Replace with Fqn
    pub path: Vec<ElementId>,
}

impl AtomId {
    pub fn typed<S>(self) -> TypedAtomId<S> {
        TypedAtomId {
            atom_id: self,
            _type: PhantomData,
        }
    }
}

#[derive(Deref)]
pub struct TypedAtomId<S> {
    #[deref]
    pub atom_id: AtomId,
    _type: PhantomData<S>,
}
