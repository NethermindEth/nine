use n9_node::atom::State;
use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {}

impl State for Config {
    type Delta = ();
    type Query = ConfigQuery;

    fn apply(&mut self, delta: Self::Delta) {}

    fn divide(&self) -> Self {
        Self {}
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum ConfigQuery {
    GetConfig { namespace: String, template: Value },
}
