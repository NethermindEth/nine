use derive_more::{Deref, DerefMut, From, Into};
use n9_node::{Dispatcher, Listener, Publisher, State, Subscriber};
use serde::{Deserialize, Serialize};
use toml::Value;

#[derive(Deref, DerefMut, From, Into)]
pub struct ConfigSub {
    listener: Listener<ConfigState>,
}

impl Subscriber for ConfigState {
    type Driver = ConfigSub;
}

#[derive(Deref, DerefMut, From, Into)]
pub struct ConfigPub {
    dispatcher: Dispatcher<ConfigState>,
}

impl Publisher for ConfigState {
    type Driver = ConfigPub;
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct ConfigState {
    config: Option<Value>,
}

impl State for ConfigState {
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
