use crate::atom::State;
use crb::agent::{Agent, AgentSession};

pub struct Recorder<S: State> {
    state: S,
}

impl<S: State> Agent for Recorder<S> {
    type Context = AgentSession<Self>;
}
