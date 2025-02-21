use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Price {
    /// The unique symbol representing the asset whose price is being queried (e.g., "BTC", "ETH").
    ticker: String,
}
