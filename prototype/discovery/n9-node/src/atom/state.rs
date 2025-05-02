// TODO: Copy the flow, but:
// 4. Add state_instance_id to send deltas directly
// 5. Provide state_instance_id with a query

use super::encoding::{FlexCodec, PackedDelta, PackedQuery, PackedState, ProtocolCodec};
use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

/// Requirements for a data fraction in a data flow.
/// `Sync` is needed to access through the `watch` channel of the `Player`.
pub trait DataFraction
where
    Self: DeserializeOwned + Serialize + Clone + Sync + Send + 'static,
{
}

impl<T> DataFraction for T where T: DeserializeOwned + Serialize + Clone + Sync + Send + 'static {}

/// Immutable state of a data flow.
pub trait State: DataFraction {
    /// `UpdateEvent` - that sent from a server to a client
    type Delta: DataFraction;

    /// `ControlEvent` - that send from a client to a server
    type Query: DataFraction;

    /// Generic name with pack.
    ///
    /// Used in spans and for rendering.
    fn class() -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Applies the delta.
    fn apply(&mut self, delta: Self::Delta);

    /// Detaches a state to send it to a client (listener)
    fn divide(&self) -> Self;

    /// Packs the state.
    fn pack_state(&self) -> Result<PackedState> {
        FlexCodec::encode(self)
    }

    /// Unpacks the state.
    fn unpack_state(data: &PackedState) -> Result<Self> {
        FlexCodec::decode(data)
    }

    /// Packs the delta.
    fn pack_delta(delta: &Self::Delta) -> Result<PackedDelta> {
        FlexCodec::encode(delta)
    }

    /// Unpacks the delta.
    fn unpack_delta(data: &PackedDelta) -> Result<Self::Delta> {
        FlexCodec::decode(data)
    }

    /// Packs the action.
    fn pack_query(query: &Self::Query) -> Result<PackedQuery> {
        FlexCodec::encode(query)
    }

    /// Unpacks the query.
    fn unpack_query(data: &PackedQuery) -> Result<Self::Query> {
        FlexCodec::decode(data)
    }
}
