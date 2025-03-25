use crb::superagent::Request;
use derive_more::From;
use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::Display;

#[derive(Serialize, Deserialize, Display, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    /// System
    Developer,
    User,
    Assistant,
    /// Function
    Tool,
}

#[derive(Serialize, Deserialize, Display, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Reason {
    Stop,
    Call,
}

impl Reason {
    pub fn is_call(&self) -> bool {
        *self == Reason::Call
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionableMessage {
    pub message: Message,
    pub reason: Reason,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: Role,
    // TODO: Use enum here, since options are different, and use `Value` for tool call response
    pub content: String,
    pub call_id: Option<CallId>,
    pub tool_calls: Vec<ToolCall>,
}

impl Message {
    pub fn content(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
            call_id: None,
            tool_calls: Vec::new(),
        }
    }
}

impl From<ActionableMessage> for Message {
    fn from(message: ActionableMessage) -> Self {
        message.message
    }
}

impl From<(CallId, ToolResult)> for Message {
    fn from((call_id, response): (CallId, ToolResult)) -> Self {
        Self {
            role: Role::Tool,
            // Use the raw `Value` here
            content: serde_json::to_string(&response.value).unwrap(),
            call_id: Some(call_id),
            tool_calls: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
}

impl From<Message> for ChatRequest {
    fn from(message: Message) -> Self {
        Self {
            messages: vec![message],
        }
    }
}

impl ChatRequest {
    pub fn with_tools(self, tools: Vec<ToolInfo>) -> ToolingChatRequest {
        ToolingChatRequest {
            messages: self.messages,
            tools,
        }
    }
}

impl ChatRequest {
    pub fn user(text: &str) -> Self {
        let message = Message {
            role: Role::User,
            content: text.to_string(),
            call_id: None,
            tool_calls: Vec::new(),
        };
        Self {
            messages: vec![message],
        }
    }
}

#[derive(Default)]
pub struct ChatResponse {
    pub messages: Vec<Message>,
}

impl From<ToolingChatResponse> for ChatResponse {
    fn from(response: ToolingChatResponse) -> Self {
        Self {
            messages: response.messages.into_iter().map(Message::from).collect(),
        }
    }
}

impl ChatResponse {
    pub fn squash(&self) -> String {
        let mut text = String::new();
        for msg in &self.messages {
            text.push_str(&msg.content);
        }
        text
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ToolingChatRequest {
    pub messages: Vec<Message>,
    pub tools: Vec<ToolInfo>,
}

impl Request for ToolingChatRequest {
    type Response = ToolingChatResponse;
}

impl ToolingChatRequest {
    pub fn squash(&self) -> String {
        let mut text = String::new();
        for msg in &self.messages {
            text.push_str(&msg.content);
        }
        text
    }
}

impl From<ChatRequest> for ToolingChatRequest {
    fn from(request: ChatRequest) -> Self {
        Self {
            messages: request.messages,
            tools: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ToolingChatResponse {
    pub messages: Vec<ActionableMessage>,
}

impl ToolingChatResponse {
    pub fn squash(&self) -> String {
        let mut text = String::new();
        for msg in &self.messages {
            text.push_str(&msg.message.content);
        }
        text
    }

    pub fn without_tools(self) -> ChatResponse {
        self.into()
    }
}

pub type CallId = String;

pub type ToolId = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolInfo {
    pub id: ToolId,
    pub meta: ToolMeta,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolMeta {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<RootSchema>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CallInfo {
    pub call_id: CallId,
    pub tool_id: ToolId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolCall {
    pub info: CallInfo,
    pub args: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, From)]
pub struct ToolResult {
    pub info: CallInfo,
    pub value: Value,
}
