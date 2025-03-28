use super::recorder::Recorder;
use crate::atom::State;
use crb::agent::Address;
use std::sync::{Arc, Weak};

pub struct Dispatcher<S: State> {
    recorder: Arc<Address<Recorder<S>>>,
}

struct DispatcherInner<S: State> {
    recorder: Address<Recorder<S>>,
}

impl<S: State> Drop for DispatcherInner<S> {
    fn drop(&mut self) {
        self.recorder.interrupt();
    }
}
