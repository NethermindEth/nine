use n9_node::atom::State;
use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    config: Option<Value>,
}

impl State for Config {
    type Delta = ConfigDelta;
    type Query = ConfigQuery;

    fn apply(&mut self, delta: Self::Delta) {
        match delta {
            ConfigDelta::NewValue { config } => {
                self.config = Some(config);
            }
        }
    }

    fn divide(&self) -> Self {
        Self { config: None }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum ConfigDelta {
    NewValue { config: Value },
}

#[derive(Deserialize, Serialize, Clone)]
pub enum ConfigQuery {
    GetConfig { namespace: String, template: Value },
}
