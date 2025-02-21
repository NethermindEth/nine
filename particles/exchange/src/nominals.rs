use n9_core::Prompt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Tickers;

impl Prompt for Tickers {
    type Output = Vec<String>;
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Price {
    /// The unique symbol representing the asset whose price is being queried
    /// (e.g., "BTC", "ETH", or pairs like "BTC-USD").
    pub ticker: String,
}

impl Prompt for Price {
    // TODO: Use BigDecimal instead
    type Output = String;
}
