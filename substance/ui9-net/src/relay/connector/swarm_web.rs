use super::behaviour::{Ui9Behaviour, PROTOCOL};
use anyhow::{Error, Result};
use libp2p::{
    core::upgrade, gossipsub, identity::Keypair, noise, websocket_websys, yamux, PeerId, Swarm,
    Transport,
};
use libp2p_request_response::{self as request_response, ProtocolSupport};
use libp2p_stream as stream;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Duration;

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

    let unique_message = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    };

    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10))
        .validation_mode(gossipsub::ValidationMode::Strict)
        .message_id_fn(unique_message)
        .build()?;

    let gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(key.clone()),
        gossipsub_config,
    )
    .map_err(Error::msg)?;

    let request_response = request_response::cbor::Behaviour::new(
        [(PROTOCOL.clone(), ProtocolSupport::Full)],
        request_response::Config::default(),
    );

    let stream = stream::Behaviour::new();

    let behaviour = Ui9Behaviour {
        gossipsub,
        request_response,
        stream,
    };

    // Create a Swarm directly without using a builder
    let swarm = Swarm::new(
        transport,
        behaviour,
        local_peer_id,
        libp2p::swarm::Config::with_wasm_executor(),
    );

    Ok(swarm)
}
