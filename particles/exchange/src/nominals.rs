use n9_core::ToolInput;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Tickers;

impl ToolInput for Tickers {
    type ToolOutput = Vec<String>;
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Price {
    /// The unique symbol representing the asset whose price is being queried
    /// (e.g., "BTC", "ETH", or pairs like "BTC-USD").
    pub ticker: String,
}

impl ToolInput for Price {
    type ToolOutput = String;
}
