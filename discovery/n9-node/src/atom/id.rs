use super::aqn::Aqn;
use derive_more::Deref;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AtomId {
    pub peer: PeerId,
    pub path: Aqn,
}

impl AtomId {
    pub fn same_peer(&self, peer: PeerId) -> bool {
        self.peer == peer
    }

    pub fn typed<S>(self) -> TypedAtomId<S> {
        TypedAtomId {
            atom: self,
            _type: PhantomData,
        }
    }
}

// TODO: Consider renaming to `Link`
#[derive(Deref)]
pub struct TypedAtomId<S> {
    #[deref]
    pub atom: AtomId,
    _type: PhantomData<S>,
}
