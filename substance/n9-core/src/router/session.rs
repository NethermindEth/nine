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
        let mut extra_messages = Vec::new();

        "Chat session"
            .in_fut(async {
                // The reasoning loop for calling tools
                loop {
                    let mut one_more_step = false;

                    "Calling the model..."
                        .in_fut(async {
                            let model = self.router.get_model().await?;
                            let tools = self.router.get_tools().await?;
                            let request_with_tools = request.clone().with_tools(tools);
                            let response = model.chat(request_with_tools).await?;

                            for message in response.messages {
                                if message.reason.is_call() {
                                    let mut callers = Vec::new();
                                    for tool_call in message.tool_calls {
                                        // One more stop to process results with a model
                                        one_more_step = true;
                                        let caller = Caller {
                                            router: self.router.clone(),
                                            tool_call,
                                        };
                                        callers.push(caller.call());
                                    }
                                    let messages = join_all(callers).await;
                                    extra_messages.extend(messages);
                                } else {
                                    extra_messages.push(message.into());
                                }
                            }
                            Ok(())
                        })
                        .await;

                    if one_more_step {
                        request.messages.extend(extra_messages.drain(..));
                    } else {
                        break;
                    }
                }
                Ok(())
            })
            .await;

        let response = ChatResponse {
            messages: extra_messages,
        };
        Ok(response)
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
            },
        }
    }

    async fn call_or_fail(mut self) -> Result<Message> {
        let id = self.tool_call.id.clone();
        format!("Calling the tool {id}")
            .in_fut(async {
                let fetcher = self.router.get_tool(self.tool_call.id);
                let link = fetcher.await?;
                let response = link.call_tool(self.tool_call.args).await?;
                let message = Message::from(response);
                Ok(message)
            })
            .await
    }
}
