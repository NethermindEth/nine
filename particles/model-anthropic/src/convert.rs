use n9_core::{ActionableMessage, Message as ModelMessage, Reason, Role as ModelRole};

#[derive(serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AnthropicRole {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(serde::Serialize)]
pub struct AnthropicMessage {
    pub role: AnthropicRole,
    pub content: String,
}

pub fn message(from: ModelMessage) -> AnthropicMessage {
    let role = match from.role {
        ModelRole::User => AnthropicRole::User,
        ModelRole::Assistant => AnthropicRole::Assistant,
        ModelRole::Developer => AnthropicRole::System,
        ModelRole::Tool => AnthropicRole::Tool,
    };

    AnthropicMessage {
        role,
        content: from.content,
    }
}

pub fn choice(from: &serde_json::Value) -> Option<ActionableMessage> {
    let role_str = from.get("role")?.as_str()?;
    let role = match role_str {
        "user" => ModelRole::User,
        "assistant" => ModelRole::Assistant,
        "system" => ModelRole::Developer,
        "tool" => ModelRole::Tool,
        _ => return None,
    };

    let content = from
        .get("content")
        .and_then(serde_json::Value::as_str)?
        .to_string();

    let message = ModelMessage {
        role,
        content,
        tool_calls: Vec::new(),
    };
    let actionable = ActionableMessage {
        message,
        // TODO: Convert the reason properly
        reason: Reason::Stop,
    };
    Some(actionable)
}
