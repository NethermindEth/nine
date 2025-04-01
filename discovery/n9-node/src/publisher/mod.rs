mod dispatcher;
mod recorder;
mod server;

pub use dispatcher::Dispatcher;
pub use recorder::{DeltaFlow, RecorderLink};
pub use server::{HubServer, HubServerLink};

use crate::atom::{AtomId, State};
use crb::core::uuid::Uuid;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

pub trait Publisher: State + Default {
    type Driver: From<Dispatcher<Self>> + Send;
}

#[derive(Deref, DerefMut)]
pub struct Pub<P: Publisher> {
    driver: P::Driver,
}

impl<P: Publisher> Pub<P> {
    pub fn new(atom_id: AtomId) -> Self {
        let state = P::default();
        let dispatcher = Dispatcher::<P>::new(atom_id, state);
        Self {
            driver: P::Driver::from(dispatcher),
        }
    }
}

/// A wrapper needed to implement a generic handler
pub struct Query<S: State> {
    pub from: StateId,
    pub query: S::Query,
}

/// An id of a replicated state.
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateId(Uuid);

impl StateId {
    pub fn unique() -> Self {
        Self(Uuid::new_v4())
    }
}
