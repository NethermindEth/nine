use libp2p::identity::Keypair;
use libp2p::PeerId;
use std::sync::LazyLock;

pub static INSTANCE_KEY: LazyLock<Key> = LazyLock::new(Key::generate);

#[derive(Debug, Clone)]
pub struct Key {
    pub pair: Keypair,
    pub peer: PeerId,
}

impl Key {
    pub fn instance() -> &'static Self {
        &*INSTANCE_KEY
    }

    /// Generates a new identity keypair
    pub fn generate() -> Self {
        let local_key = Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        log::info!("Local peer id: {local_peer_id}");
        Self {
            pair: local_key,
            peer: local_peer_id,
        }
    }
}
