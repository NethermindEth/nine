use n9_core::Prompt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Tickers {
    // Must be a struct to fit open ai schema
}

// TODO: Rename `Prompt` to `Action`
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
        price information, ensuring up-to-date market data for further processing or display."
    }
}

pub type OrderId = u64;

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Order {
    /// The unique symbol representing the asset for which the order is placed (e.g., "BTC-USD")
    pub ticker: String,
    /// The price at which the order is placed. If not specified, the market price will be used.
    pub price: Option<f64>,
    /// The quantity of the asset in the order
    pub amount: f64,
    /// The type of order: `Buy` or `Sell`
    pub order_type: OrderType,
}

impl Prompt for Order {
    type Output = u64;

    fn description() -> &'static str {
        "This function places a trade order on an exchange with a specified price and quantity.
        By providing an asset ticker, price, amount, and order type, the function interacts
        with the exchange API to execute a trade order."
    }
}
