use derive_more::{Deref, DerefMut, From, Into};
use n9_node::{DataFraction, Dispatcher, Listener, Publisher, State, Subscriber};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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

#[derive(Deserialize, Serialize, Clone)]
pub struct ConfigState<T = Value> {
    config: Option<T>,
}

impl<T> Default for ConfigState<T> {
    fn default() -> Self {
        Self { config: None }
    }
}

impl<T> State for ConfigState<T>
where
    T: DataFraction,
{
    type Delta = ConfigDelta;
    type Query = ConfigQuery;

    fn apply(&mut self, delta: Self::Delta) {
        match delta {
            ConfigDelta::NewValue { config } => {
                let value = config.try_into();
                match value {
                    Ok(value) => {
                        self.config = Some(value);
                    }
                    Err(err) => {}
                }
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
