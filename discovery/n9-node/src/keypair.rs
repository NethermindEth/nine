use libp2p::identity::Keypair;
use libp2p::PeerId;

#[derive(Debug, Clone)]
pub struct Key {
    pub pair: Keypair,
    pub peer: PeerId,
}

impl Key {
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
