use super::client::HubClient;
use super::player::{GetDeltasChannel, Player, SendQuery};
use super::SubEvent;
use crate::atom::{AtomId, State};
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
    pub fn new(atom_id: AtomId) -> Self {
        let player = Player::new(atom_id.typed());
        let player = HubClient::spawn_player::<S>(player);
        let inner = ListenerInner::from(player);
        Self {
            inner: Arc::new(inner),
        }
    }

    pub async fn receiver(&mut self) -> Result<mpsc::UnboundedReceiver<SubEvent<S>>> {
        let request = GetDeltasChannel::new();
        self.inner
            .player
            .interact(request)
            .await
            .map_err(Error::from)
    }

    pub async fn events(&mut self) -> Result<Drainer<SubEvent<S>>> {
        self.receiver().await.map(drainer_from_mpsc)
    }

    pub fn query(&self, query: S::Query) -> Result<()> {
        let event = SendQuery::new(query);
        self.inner.player.event(event)
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
