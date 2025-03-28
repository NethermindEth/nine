use crate::ids::AtomId;
use libp2p_request_response::{self as request_response};
use serde::{Deserialize, Serialize};

pub type Behaviour = request_response::cbor::Behaviour<Request, Response>;

pub type Event = request_response::Event<Request, Response>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Request {
    pub atom_id: AtomId,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Response {}
