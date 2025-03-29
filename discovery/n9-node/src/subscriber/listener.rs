use super::player::{Deltas, Player};
use super::StateEvent;
use crate::atom::State;
use crate::utils::drainer_from_mpsc;
use anyhow::{Error, Result};
use crb::agent::Address;
use crb::core::mpsc;
use crb::superagent::{Drainer, InteractExt};
use derive_more::From;
use std::sync::Arc;

pub struct Listener<S: State> {
    inner: Arc<ListenerInner<S>>,
}

impl<S: State> Listener<S> {
    pub async fn receiver(&mut self) -> Result<mpsc::UnboundedReceiver<StateEvent<S>>> {
        let request = Deltas::new();
        self.inner
            .player
            .interact(request)
            .await
            .map_err(Error::from)
    }

    pub async fn events(&mut self) -> Result<Drainer<StateEvent<S>>> {
        self.receiver().await.map(drainer_from_mpsc)
    }
}

#[derive(From)]
struct ListenerInner<S: State> {
    player: Address<Player<S>>,
}

impl<S: State> Drop for ListenerInner<S> {
    fn drop(&mut self) {
        self.player.interrupt();
    }
}
