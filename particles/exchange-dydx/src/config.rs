use dydx::config::ClientConfig;
use dydx::indexer::{IndexerConfig, RestConfig, SockConfig};
use n9_core::Config;
use serde::{Deserialize, Serialize};
use std::num::NonZero;

#[derive(Deserialize, Serialize)]
pub struct DyDxConfig {
    pub rest_endpoint: String,
}

impl Config for DyDxConfig {
    const NAMESPACE: &str = "dydx";

    fn template() -> Self {
        Self {
            rest_endpoint: "https://indexer.v4testnet.dydx.exchange".into(),
        }
    }
}

impl DyDxConfig {
    pub fn extract(&self) -> IndexerConfig {
        IndexerConfig {
            rest: RestConfig {
                endpoint: self.rest_endpoint.clone(),
            },
            sock: SockConfig {
                endpoint: String::new(),
                timeout: 2_000,
                rate_limit: NonZero::new(1).unwrap(),
            },
        }
    }
}
