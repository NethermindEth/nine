use super::behaviour::NineBehaviour;
use super::keypair::Key;
use anyhow::Result;
use futures::future::Either;
use libp2p::{
    core::muxing::StreamMuxerBox, core::upgrade, noise, tcp, websocket, yamux, Swarm, Transport,
};
use std::time::Duration;

pub(super) async fn swarm(key: &Key) -> Result<Swarm<NineBehaviour>> {
    // Create Noise for encryption
    let noise = noise::Config::new(&key.pair)?;
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

    let behaviour = NineBehaviour::new(&key.pair)?;

    // Create a Swarm directly without using a builder
    let swarm = Swarm::new(
        transport,
        behaviour,
        key.peer,
        libp2p::swarm::Config::with_tokio_executor(),
    );

    Ok(swarm)
}
