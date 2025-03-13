use libp2p::identity::Keypair;
use libp2p::PeerId;

pub struct Key {
    pub pair: Keypair,
    pub peer: PeerId,
}

impl Key {
    pub fn generate() -> Self {
        // Generate a new identity keypair
        let local_key = Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        log::info!("Local peer id: {local_peer_id}");
        Self {
            pair: local_key,
            peer: local_peer_id,
        }
    }
}
