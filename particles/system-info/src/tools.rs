use n9_core::Prompt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ToolsList {}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
}

impl Prompt for ToolsList {
    type Output = Vec<ToolInfo>;

    fn description() -> &'static str {
        "The **ToolsList** is a utility that provides a real-time list of available
        AI tools within a system. It helps users quickly identify, access, and diagnose
        tool availability for troubleshooting and workflow optimization. With categorized
        listings and self-diagnostics capabilities, it ensures seamless integration
        and efficient task execution."
    }
}
