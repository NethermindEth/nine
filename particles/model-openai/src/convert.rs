use anyhow::{anyhow, Result};
use async_openai::types::*;
use n9_core::{
    ActionableMessage, Message as MessageN9, Reason, Role as RoleN9, ToolCall, ToolInfo,
};
use schemars::schema::RootSchema;
use serde_json::{json, Value};

// REQUESTS

/// From N9 to OpenAI
pub fn message(from: MessageN9) -> Result<ChatCompletionRequestMessage> {
    let msg = match from.role {
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
            let calls = from
                .tool_calls
                .into_iter()
                .map(tool_call_to_chat)
                .collect::<Result<_>>()?;
            message.tool_calls = Some(calls);
            ChatCompletionRequestMessage::from(message)
        }
        RoleN9::Tool => {
            let mut message = ChatCompletionRequestToolMessage::default();
            let content = ChatCompletionRequestToolMessageContent::Text(from.content);
            message.content = content;
            // TODO: Use enum instead
            message.tool_call_id = from.call_id.unwrap_or_default();
            ChatCompletionRequestMessage::from(message)
        }
    };
    Ok(msg)
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
    let mut value = serde_json::to_value(&root_schema.schema).unwrap_or_default();

    // OpenAI is sensitive to the schema. It requires:
    // - `type` is `object`
    // - `paameters` always exist
    // But, the problem since the scheme is overriden, if used a unit type,
    // it will be represented as a struct and parameters won't be deserialized.
    // In short: `struct`s must be used!

    // Ensure "type" is not null and is an object
    if let Some(type_value) = value.get("type").and_then(Value::as_str) {
        if type_value != "object" {
            value["type"] = json!("object");
        }
    } else {
        value["type"] = json!("object");
    }

    // Ensure "properties" exists and is an object
    if !value.get("properties").map_or(false, Value::is_object) {
        value["properties"] = json!({});
    }

    value
}

// RESPONSES

pub fn choice(from: ChatChoice) -> Result<ActionableMessage> {
    let role = match from.message.role {
        Role::System => RoleN9::Developer,
        Role::User => RoleN9::User,
        Role::Assistant => RoleN9::Assistant,
        Role::Tool => RoleN9::Tool,
        other => {
            return Err(anyhow!("Unsupported role: {other}"));
        }
    };
    let content = from.message.content.unwrap_or_default();
    let calls = from.message.tool_calls.unwrap_or_default();
    let tool_calls = calls
        .into_iter()
        .map(tool_call_convert)
        .collect::<Result<_>>()?;
    let message = MessageN9 {
        role,
        content,
        call_id: None,
        tool_calls,
    };
    let actionable = ActionableMessage {
        message,
        reason: reason(from.finish_reason),
    };
    Ok(actionable)
}

fn reason(finish_reason: Option<FinishReason>) -> Reason {
    if let Some(reason) = finish_reason {
        match reason {
            FinishReason::Stop => Reason::Stop,
            FinishReason::Length => Reason::Stop,
            FinishReason::ContentFilter => Reason::Stop,
            FinishReason::ToolCalls => Reason::Call,
            FinishReason::FunctionCall => Reason::Call,
        }
    } else {
        Reason::Stop
    }
}

fn tool_call_convert(call: ChatCompletionMessageToolCall) -> Result<ToolCall> {
    let args = serde_json::from_str(&call.function.arguments)?;
    Ok(ToolCall {
        call_id: call.id,
        tool_id: call.function.name.into(),
        args,
    })
}

fn tool_call_to_chat(tool_call: ToolCall) -> Result<ChatCompletionMessageToolCall> {
    let arguments = serde_json::to_string(&tool_call.args)?;
    Ok(ChatCompletionMessageToolCall {
        id: tool_call.call_id,
        r#type: ChatCompletionToolType::Function,
        function: FunctionCall {
            name: tool_call.tool_id,
            arguments,
        },
    })
}

/*
let file = std::fs::File::create("output.json").unwrap();
let writer = std::io::BufWriter::new(file);
serde_json::to_writer_pretty(writer, &value).unwrap();
*/
