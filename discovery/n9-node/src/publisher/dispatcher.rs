use super::recorder::Recorder;
use super::Query;
use crate::atom::State;
use anyhow::Result;
use crb::agent::Address;
use crb::core::mpsc;
use std::sync::Arc;

pub struct Dispatcher<S: State> {
    recorder: Arc<Address<Recorder<S>>>,
}

impl<S: State> Dispatcher<S> {
    pub async fn queries(&mut self) -> Result<mpsc::UnboundedReceiver<Query<S>>> {
        // TODO: Send a request to get a receiver
        todo!()
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
