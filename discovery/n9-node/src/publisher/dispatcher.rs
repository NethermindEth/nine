use super::recorder::{Delta, Queries, Recorder};
use super::server::HubServer;
use super::Query;
use crate::atom::State;
use anyhow::{Error, Result};
use crb::agent::Address;
use crb::core::mpsc;
use crb::superagent::InteractExt;
use std::sync::Arc;

pub struct Dispatcher<S: State> {
    recorder: Arc<Address<Recorder<S>>>,
}

impl<S: State> Dispatcher<S> {
    pub fn new(state: S) -> Self {
        let recorder = HubServer::spawn_recorder(state);
        Self {
            recorder: Arc::new(recorder),
        }
    }

    pub async fn queries(&mut self) -> Result<mpsc::UnboundedReceiver<Query<S>>> {
        let request = Queries::new();
        self.recorder.interact(request).await.map_err(Error::from)
    }

    pub fn event(&self, delta: S::Delta) -> Result<()> {
        let event = Delta::new(delta);
        self.recorder.event(event)
    }
}

struct DispatcherInner<S: State> {
    recorder: Address<Recorder<S>>,
}

impl<S: State> Drop for DispatcherInner<S> {
    fn drop(&mut self) {
        self.recorder.interrupt();
    }
}
