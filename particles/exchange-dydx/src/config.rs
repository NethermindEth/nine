use dydx::config::ClientConfig;
use dydx::indexer::{IndexerConfig, RestConfig, SockConfig};
use n9_core::Config;
use serde::{Deserialize, Serialize};
use std::num::NonZero;

const TEMPLATE: &str = include_str!("../template.toml");

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct DyDxConfig {
    pub config: ClientConfig,
}

impl Config for DyDxConfig {
    const NAMESPACE: &str = "dydx";

    fn template() -> Self {
        toml::from_str(TEMPLATE).expect("Wrong config template")
    }
}
