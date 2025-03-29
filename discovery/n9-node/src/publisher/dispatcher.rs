use super::recorder::{Queries, Recorder, SendDelta};
use super::server::HubServer;
use super::{Query, StateId};
use crate::atom::State;
use crate::utils::drainer_from_mpsc;
use anyhow::{Error, Result};
use crb::agent::Address;
use crb::core::mpsc;
use crb::superagent::{Drainer, InteractExt};
use derive_more::From;
use std::sync::Arc;

#[derive(Clone)]
pub struct Dispatcher<S: State> {
    inner: Arc<DispatcherInner<S>>,
}

impl<S: State> Dispatcher<S> {
    pub fn new(state: S) -> Self {
        let recorder = HubServer::spawn_recorder(state);
        let inner = DispatcherInner::from(recorder);
        Self {
            inner: Arc::new(inner),
        }
    }

    pub async fn receiver(&mut self) -> Result<mpsc::UnboundedReceiver<Query<S>>> {
        let request = Queries::new();
        self.inner
            .recorder
            .interact(request)
            .await
            .map_err(Error::from)
    }

    pub async fn queries(&mut self) -> Result<Drainer<Query<S>>> {
        self.receiver().await.map(drainer_from_mpsc)
    }

    pub fn broadcast(&self, delta: S::Delta) -> Result<()> {
        let event = SendDelta::new(None, delta);
        self.inner.recorder.event(event)
    }

    pub fn direct(&self, state_id: StateId, delta: S::Delta) -> Result<()> {
        let event = SendDelta::new(Some(state_id), delta);
        self.inner.recorder.event(event)
    }
}

#[derive(From)]
struct DispatcherInner<S: State> {
    recorder: Address<Recorder<S>>,
}

impl<S: State> Drop for DispatcherInner<S> {
    fn drop(&mut self) {
        self.recorder.interrupt();
    }
}
