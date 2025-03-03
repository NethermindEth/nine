use super::behaviour::Ui9Behaviour;
use anyhow::Result;
use futures::future::Either;
use libp2p::{
    core::muxing::StreamMuxerBox, core::upgrade, identity::Keypair, noise, tcp, websocket, yamux,
    PeerId, Swarm, Transport,
};
use std::time::Duration;

pub(super) async fn swarm() -> Result<Swarm<Ui9Behaviour>> {
    // Generate a new identity keypair
    let local_key = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    log::info!("Local peer id: {local_peer_id}");
    let key = local_key.clone();

    // Create Noise for encryption
    let noise = noise::Config::new(&key)?;
    let mplex = yamux::Config::default();

    // Create TCP transport with tokio
    let tcp_transport = tcp::tokio::Transport::new(tcp::Config::default())
        .upgrade(upgrade::Version::V1)
        .authenticate(noise.clone())
        .multiplex(mplex.clone())
        .timeout(Duration::from_secs(20))
        .boxed();

    // Create WebSocket transport with tokio
    let ws_transport = websocket::WsConfig::new(tcp::tokio::Transport::new(tcp::Config::default()))
        .upgrade(upgrade::Version::V1)
        .authenticate(noise.clone())
        .multiplex(mplex.clone())
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

    let behaviour = Ui9Behaviour::new(&key)?;

    // Create a Swarm directly without using a builder
    let swarm = Swarm::new(
        transport,
        behaviour,
        local_peer_id,
        libp2p::swarm::Config::with_tokio_executor(),
    );

    Ok(swarm)
}
