use n9_core::Prompt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Tickers;

impl Prompt for Tickers {
    type Output = Vec<String>;

    fn description() -> &'static str {
        "This function retrieves a list of all available stock or cryptocurrency tickers from a specified exchange.
        It allows an AI system to access up-to-date market symbols for various financial instruments,
        such as pairs of cryptocurrencies or futures."
    }
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

    fn description() -> &'static str {
        "This function fetches the current market price of a specified asset from an exchange.
        By providing a valid asset ticker, the function queries the DEX's pricing endpoint to retrieve real-time
        price information, ensuring up-to-date market data for further processing or display.".into()
    }
}
