use crb::superagent::Request;
use schemars::schema::RootSchema;
use serde_json::Value;

pub enum Role {
    /// System
    Developer,
    User,
    Assistant,
    /// Function
    Tool,
}

pub enum Reason {
    Stop,
    Call,
}

pub struct ActionableMessage {
    pub message: Message,
    pub reason: Reason,
    pub tool_calls: Vec<ToolCall>,
}

pub struct ToolCall {
    pub id: ToolId,
    pub args: Value,
}

pub struct Message {
    pub role: Role,
    pub content: String,
}

impl From<ActionableMessage> for Message {
    fn from(message: ActionableMessage) -> Self {
        message.message
    }
}

#[derive(Default)]
pub struct ChatRequest {
    pub messages: Vec<Message>,
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
        };
        Self {
            messages: vec![message],
        }
    }
}

impl Request for ChatRequest {
    type Response = ChatResponse;
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

#[derive(Default)]
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

pub type ToolId = String;

#[derive(Clone, Debug)]
pub struct ToolInfo {
    pub id: ToolId,
    pub meta: ToolMeta,
}

#[derive(Debug, Clone)]
pub struct ToolMeta {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Option<RootSchema>,
}
