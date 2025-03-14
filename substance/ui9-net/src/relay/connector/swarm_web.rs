use super::behaviour::Ui9Behaviour;
use crate::relay::keypair::Key;
use anyhow::Result;
use libp2p::{core::upgrade, noise, websocket_websys, yamux, Swarm, Transport};

pub(super) async fn swarm(key: &Key) -> Result<Swarm<Ui9Behaviour>> {
    // Create Noise for encryption
    let noise = noise::Config::new(&key.pair)?;
    let mplex = yamux::Config::default();

    let ws_transport = websocket_websys::Transport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise.clone())
        .multiplex(mplex.clone())
        .boxed();

    // Combine transports: WebSocket
    let transport = ws_transport;

    let behaviour = Ui9Behaviour::new(&key.pair)?;

    // Create a Swarm directly without using a builder
    let swarm = Swarm::new(
        transport,
        behaviour,
        key.peer,
        libp2p::swarm::Config::with_wasm_executor(),
    );

    Ok(swarm)
}
