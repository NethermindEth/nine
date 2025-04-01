mod dispatcher;
mod recorder;
mod server;

pub use recorder::{DeltaFlow, RecorderLink};
pub use server::{HubServer, HubServerLink};

use crate::atom::State;
use crb::core::uuid::Uuid;
use serde::{Deserialize, Serialize};

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
