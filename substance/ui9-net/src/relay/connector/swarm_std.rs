use crate::relay::connector::Ui9Behaviour;
use anyhow::{Error, Result};
use futures::future::Either;
use libp2p::{
    core::muxing::StreamMuxerBox, core::upgrade, gossipsub, identity::Keypair, mdns, noise, tcp,
    websocket, yamux, PeerId, StreamProtocol, Swarm, Transport,
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

    // Create TCP transport with tokio
    let tcp_transport = tcp::tokio::Transport::new(tcp::Config::default())
        .upgrade(upgrade::Version::V1)
        .authenticate(noise.clone())
        .multiplex(yamux::Config::default())
        .timeout(Duration::from_secs(20))
        .boxed();

    // Create WebSocket transport with tokio
    let ws_transport = websocket::WsConfig::new(tcp::tokio::Transport::new(tcp::Config::default()))
        .upgrade(upgrade::Version::V1)
        .authenticate(noise.clone())
        .multiplex(yamux::Config::default())
        .timeout(Duration::from_secs(20))
        .boxed();

    // Create QUIC transport with tokio
    /*
    let quic_transport = quic::tokio::Transport::new(quic::Config::new(&local_key));
    */

    // Combine transports: TCP or WebSocket or QUIC
    let transport = tcp_transport
        .or_transport(ws_transport)
        //.or_transport(quic_transport)
        .map(|either_output, _connected_point| {
            // Transport selection
            match either_output {
                Either::Left((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
                Either::Right((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
            }
        })
        .boxed();

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

    let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

    let request_response = request_response::cbor::Behaviour::new(
        [(
            StreamProtocol::new("/ui9-trace/0.0.1"),
            ProtocolSupport::Full,
        )],
        request_response::Config::default(),
    );

    let stream = stream::Behaviour::new();

    let behaviour = Ui9Behaviour {
        gossipsub,
        #[cfg(feature = "mdns")]
        mdns,
        request_response,
        stream,
    };

    // Create a Swarm directly without using a builder
    let swarm = Swarm::new(
        transport,
        behaviour,
        local_peer_id,
        libp2p::swarm::Config::with_tokio_executor(),
    );

    Ok(swarm)
}
