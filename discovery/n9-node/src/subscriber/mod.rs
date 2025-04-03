mod client;
mod listener;
mod player;

pub use client::HubClient;
pub use listener::Listener;

use crate::atom::{AtomId, State};
use crb::core::watch;
use derive_more::{Deref, DerefMut};
use std::ops::DerefMut;

pub trait Subscriber: State {
    type Driver: From<Listener<Self>> + DerefMut<Target = Listener<Self>> + Send;
}

#[derive(Deref, DerefMut)]
pub struct Sub<P: Subscriber> {
    driver: P::Driver,
}

impl<P: Subscriber> Sub<P> {
    pub fn connect(atom_id: AtomId) -> Self {
        let listener = Listener::<P>::new(atom_id);
        Self {
            driver: P::Driver::from(listener),
        }
    }
}

#[derive(Debug)]
pub enum SubEvent<S: State> {
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
