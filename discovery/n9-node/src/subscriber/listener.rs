use super::player::Player;
use crate::atom::State;
use crb::agent::Address;
use std::sync::Arc;

pub struct Listener<S: State> {
    player: Arc<Address<Player<S>>>,
}
