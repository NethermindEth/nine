use libp2p::{gossipsub, swarm::NetworkBehaviour, StreamProtocol};
use libp2p_request_response::{self as request_response};
use libp2p_stream as stream;

#[cfg(feature = "mdns")]
use libp2p::mdns;

pub static PROTOCOL: StreamProtocol = StreamProtocol::new("/ui9-trace/0.0.1");

#[derive(NetworkBehaviour)]
pub struct Ui9Behaviour {
    pub gossipsub: gossipsub::Behaviour,
    #[cfg(feature = "mdns")]
    pub mdns: mdns::tokio::Behaviour,
    pub request_response: request_response::cbor::Behaviour<(), ()>,
    pub stream: stream::Behaviour,
}
