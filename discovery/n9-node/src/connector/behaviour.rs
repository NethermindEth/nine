use super::protocol;
use anyhow::{Error, Result};
use libp2p::{gossipsub, identity::Keypair, swarm::NetworkBehaviour, StreamProtocol};
use libp2p_request_response::{self as request_response, ProtocolSupport};
use libp2p_stream as stream;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Duration;

#[cfg(feature = "mdns")]
use libp2p::mdns;

pub static PROTOCOL: StreamProtocol = StreamProtocol::new("/nine/0.0.1");

#[derive(NetworkBehaviour)]
pub struct NineBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    #[cfg(feature = "mdns")]
    pub mdns: mdns::tokio::Behaviour,
    pub request_response: protocol::Behaviour,
    pub stream: stream::Behaviour,
}

impl NineBehaviour {
    pub fn new(key: &Keypair) -> Result<Self> {
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

        #[cfg(feature = "mdns")]
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

        let request_response = request_response::cbor::Behaviour::new(
            [(PROTOCOL.clone(), ProtocolSupport::Full)],
            request_response::Config::default(),
        );

        let stream = stream::Behaviour::new();

        Ok(Self {
            gossipsub,
            #[cfg(feature = "mdns")]
            mdns,
            request_response,
            stream,
        })
    }
}
