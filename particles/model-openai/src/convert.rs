use anyhow::Result;
use async_openai::types::*;
use n9_core::{ActionableMessage, Message as MessageN9, Role as RoleN9, ToolCall, ToolInfo};
use schemars::schema::RootSchema;
use serde_json::Value;

// REQUESTS

/// From N9 to OpenAI
pub fn message(from: MessageN9) -> ChatCompletionRequestMessage {
    match from.role {
        RoleN9::Developer => {
            let mut message = ChatCompletionRequestSystemMessage::default();
            let content = ChatCompletionRequestSystemMessageContent::Text(from.content);
            message.content = content;
            ChatCompletionRequestMessage::from(message)
        }
        RoleN9::User => {
            let mut message = ChatCompletionRequestUserMessage::default();
            let content = ChatCompletionRequestUserMessageContent::Text(from.content);
            message.content = content;
            ChatCompletionRequestMessage::from(message)
        }
        RoleN9::Assistant => {
            let mut message = ChatCompletionRequestAssistantMessage::default();
            let content = ChatCompletionRequestAssistantMessageContent::Text(from.content);
            message.content = Some(content);
            ChatCompletionRequestMessage::from(message)
        }
        RoleN9::Tool => {
            let mut message = ChatCompletionRequestToolMessage::default();
            let content = ChatCompletionRequestToolMessageContent::Text(from.content);
            message.content = content;
            ChatCompletionRequestMessage::from(message)
        }
    }
}

pub fn tool(info: ToolInfo) -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: info.id,
            description: info.meta.description,
            parameters: info.meta.parameters.map(schema),
            strict: None,
        },
    }
}

pub fn schema(root_schema: RootSchema) -> Value {
    serde_json::to_value(&root_schema.schema).unwrap_or_default()
}

// RESPONSES

pub fn choice(from: ChatChoice) -> Option<ActionableMessage> {
    let role = match from.message.role {
        Role::System => RoleN9::Developer,
        Role::User => RoleN9::User,
        Role::Assistant => RoleN9::Assistant,
        Role::Tool => RoleN9::Tool,
        _ => {
            return None;
        }
    };
    let content = from.message.content?;
    let message = MessageN9 { role, content };
    let calls = from.message.tool_calls.unwrap_or_default();
    // TODO: Return error if failed
    let tool_calls: Result<Vec<_>> = calls.into_iter().map(tool_call_convert).collect();
    let actionable = ActionableMessage {
        message,
        tool_calls: tool_calls.ok()?,
    };
    Some(actionable)
}

fn tool_call_convert(call: ChatCompletionMessageToolCall) -> Result<ToolCall> {
    let args = serde_json::from_str(&call.function.arguments)?;
    Ok(ToolCall {
        id: call.function.name.into(),
        args,
    })
}

/*
let file = std::fs::File::create("output.json").unwrap();
let writer = std::io::BufWriter::new(file);
serde_json::to_writer_pretty(writer, &value).unwrap();
*/
