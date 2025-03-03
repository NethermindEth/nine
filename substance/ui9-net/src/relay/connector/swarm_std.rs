use crate::relay::connector::Ui9Behaviour;
use anyhow::Result;
use libp2p::{
    gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, StreamProtocol, Swarm, SwarmBuilder,
};
use libp2p_request_response::{self as request_response, ProtocolSupport};
use libp2p_stream as stream;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Duration;

pub async fn swarm() -> Result<Swarm<Ui9Behaviour>> {
    let swarm = SwarmBuilder::with_new_identity();

    let swarm = swarm
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_websocket(noise::Config::new, yamux::Config::default)
        .await?;
    // .with_dns()
    // .with_quic();

    let swarm = swarm.with_behaviour(|key| {
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
        )?;

        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

        let request_response = request_response::cbor::Behaviour::new(
            [(
                StreamProtocol::new("/ui9-trace/0.0.1"),
                ProtocolSupport::Full,
            )],
            request_response::Config::default(),
        );

        let stream = stream::Behaviour::new();

        Ok(Ui9Behaviour {
            gossipsub,
            #[cfg(feature = "mdns")]
            mdns,
            request_response,
            stream,
        })
    })?;

    let swarm = swarm.build();
    Ok(swarm)
}
