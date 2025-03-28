//! The basic encoding routines.

use anyhow::{Error, Result};
use derive_more::{From, Into};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

pub trait ProtocolData: Serialize + DeserializeOwned + Send + 'static {}

impl<T> ProtocolData for T where T: Serialize + DeserializeOwned + Send + 'static {}

pub trait ProtocolCodec: Send {
    fn encode<T, P: From<Vec<u8>>>(value: &T) -> Result<P>
    where
        T: ProtocolData + ?Sized,
        P: From<Vec<u8>>;

    fn decode<T, P>(data: T) -> Result<P>
    where
        T: AsRef<[u8]>,
        P: ProtocolData;
}

#[derive(Debug)]
pub struct FlexCodec;

impl ProtocolCodec for FlexCodec {
    fn encode<T, P: From<Vec<u8>>>(value: &T) -> Result<P, Error>
    where
        T: ProtocolData + ?Sized,
        P: From<Vec<u8>>,
    {
        flexbuffers::to_vec(value).map(P::from).map_err(Error::from)
    }

    fn decode<T, P>(data: T) -> Result<P>
    where
        T: AsRef<[u8]>,
        P: ProtocolData,
    {
        flexbuffers::from_slice(data.as_ref()).map_err(Error::from)
    }
}

macro_rules! packed {
    ($name:ident) => {
        #[derive(Clone, From, Into, Serialize, Deserialize, PartialEq, Eq)]
        pub struct $name(pub Vec<u8>);

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("size", &self.0.len())
                    .finish()
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }
    };
}

packed!(PackedState);
packed!(PackedEvent);
packed!(PackedAction);
