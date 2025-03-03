use super::behaviour::Ui9Behaviour;
use anyhow::Result;
use libp2p::{
    core::upgrade, identity::Keypair, noise, websocket_websys, yamux, PeerId, Swarm, Transport,
};

pub(super) async fn swarm() -> Result<Swarm<Ui9Behaviour>> {
    // Generate a new identity keypair
    let local_key = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id}");
    let key = local_key.clone();

    // Create Noise for encryption
    let noise = noise::Config::new(&key)?;
    let mplex = yamux::Config::default();

    let ws_transport = websocket_websys::Transport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise.clone())
        .multiplex(mplex.clone())
        .boxed();

    // Combine transports: WebSocket
    let transport = ws_transport;

    let behaviour = Ui9Behaviour::new(&key)?;

    // Create a Swarm directly without using a builder
    let swarm = Swarm::new(
        transport,
        behaviour,
        local_peer_id,
        libp2p::swarm::Config::with_wasm_executor(),
    );

    Ok(swarm)
}
