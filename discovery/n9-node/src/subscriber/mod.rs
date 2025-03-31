mod client;
mod listener;
mod player;

use crate::atom::State;
use crb::core::watch;

#[derive(Debug)]
pub enum StateEvent<S: State> {
    State(Projection<S>),
    Delta(S::Delta),
    Lost,
}

#[derive(Debug)]
pub struct Projection<T> {
    state_rx: watch::Receiver<T>,
}

impl<T> Projection<T> {
    pub fn new(state: T) -> (Self, watch::Sender<T>) {
        let (state_tx, state_rx) = watch::channel(state);
        (Self { state_rx }, state_tx)
    }

    pub fn borrow(&self) -> watch::Ref<T> {
        self.state_rx.borrow()
    }
}
