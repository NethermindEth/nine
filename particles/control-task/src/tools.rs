use n9_core::Prompt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TasksList {}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct TaskInfo {
    pub interval_sec: u64,
    pub prompt: String,
}

impl Prompt for TasksList {
    type Output = Vec<TaskInfo>;

    fn description() -> &'static str {
        ""
    }
}
