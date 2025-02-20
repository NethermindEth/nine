use super::tool::ToolLink;
use super::types::{ChatRequest, ChatResponse, Message, Role, ToolCall};
use super::RouterLink;
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, Next, StopAddress};
use crb::superagent::{Fetcher, InteractExt, OnRequest};
use derive_more::{Deref, DerefMut};
use futures::future::join_all;
use serde_json::Value;
use ui9_dui::{Operate, Operation};

#[derive(Deref, DerefMut)]
pub struct SessionLink {
    address: StopAddress<ReasoningSession>,
}

impl From<Address<ReasoningSession>> for SessionLink {
    fn from(address: Address<ReasoningSession>) -> Self {
        Self {
            address: address.to_stop_address(),
        }
    }
}

impl SessionLink {
    pub fn chat(&self, request: ChatRequest) -> Fetcher<ChatResponse> {
        self.interact(request)
    }
}

pub struct ReasoningSession {
    router: RouterLink,
}

impl ReasoningSession {
    pub fn new(router: RouterLink) -> Self {
        Self { router }
    }
}

impl Agent for ReasoningSession {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::events()
    }
}

#[async_trait]
impl OnRequest<ChatRequest> for ReasoningSession {
    async fn on_request(
        &mut self,
        mut request: ChatRequest,
        ctx: &mut Context<Self>,
    ) -> Result<ChatResponse> {
        let extra_messages = RequestPerformer::new(self.router.clone(), request)
            .entrypoint()
            .await;
        let response = ChatResponse {
            messages: extra_messages,
        };
        Ok(response)
    }
}

struct RequestPerformer {
    router: RouterLink,
    extra_messages: Vec<Message>,
    request: ChatRequest,
    one_more_step: bool,
}

impl RequestPerformer {
    fn new(router: RouterLink, request: ChatRequest) -> Self {
        Self {
            router,
            extra_messages: Vec::new(),
            request,
            one_more_step: false,
        }
    }

    async fn entrypoint(mut self) -> Vec<Message> {
        "Chat session".in_fut(self.chat_session()).await;
        self.extra_messages
    }

    async fn chat_session(&mut self) -> Result<()> {
        // The reasoning loop for calling tools
        loop {
            self.one_more_step = false;

            "Calling the model...".in_fut(self.calling_model()).await;

            if self.one_more_step {
                self.request.messages.extend(self.extra_messages.drain(..));
            } else {
                break;
            }
        }
        Ok(())
    }

    async fn calling_model(&mut self) -> Result<()> {
        let model = self.router.get_model().await?;
        let tools = self.router.get_tools().await?;
        let request_with_tools = self.request.clone().with_tools(tools);
        let response = model.chat(request_with_tools).await?;

        for message in response.messages {
            let mut callers = Vec::new();

            if message.reason.is_call() {
                for tool_call in message.message.tool_calls.clone() {
                    // One more stop to process results with a model
                    self.one_more_step = true;
                    let caller = Caller {
                        router: self.router.clone(),
                        tool_call,
                    };
                    callers.push(caller.call());
                }
            }

            // TODO: Should I keep the order?
            self.extra_messages.push(message.into());

            let messages = join_all(callers).await;
            self.extra_messages.extend(messages);
        }
        Ok(())
    }
}

struct Caller {
    router: RouterLink,
    tool_call: ToolCall,
}

impl Caller {
    async fn call(self) -> Message {
        match self.call_or_fail().await {
            Ok(message) => message,
            Err(err) => Message {
                role: Role::Tool,
                content: format!("Tool failed: {err}"),
                tool_calls: Vec::new(),
            },
        }
    }

    async fn call_or_fail(mut self) -> Result<Message> {
        let id = self.tool_call.tool_id.clone();
        format!("Calling the tool {id}")
            .in_fut(async {
                let fetcher = self.router.get_tool(self.tool_call.tool_id);
                let link = fetcher.await?;
                let response = link.call_tool(self.tool_call.args).await?;
                let message = Message::from(response);
                Ok(message)
            })
            .await
    }
}
