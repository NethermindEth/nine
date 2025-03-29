mod dispatcher;
mod recorder;
mod server;

use crate::atom::State;

/// A wrapper needed to implement a generic handler
pub struct Query<S: State> {
    pub query: S::Query,
}

/// An id of a replicated state.
pub struct StateId(usize);
