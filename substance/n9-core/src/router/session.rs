use super::types::{ChatRequest, ChatResponse, Message, Reason};
use super::RouterLink;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, AgentSession, Context, Next, StopAddress};
use crb::superagent::{Fetcher, InteractExt, OnRequest};
use derive_more::{Deref, DerefMut};
use ui9_dui::Operation;

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

        // The reasoning loop for calling tools
        loop {
            let mut one_more_step = false;

            let model = self.router.get_model().await?;
            let tools = self.router.get_tools().await?;
            let request_with_tools = request.clone().with_tools(tools);
            let response = model.chat(request_with_tools).await?;

            for message in response.messages {
                if message.reason.is_call() {
                    for tool_call in message.tool_calls {
                        one_more_step = true;
                        // TODO: Wrap that into a closure
                        let tool_id = tool_call.id.clone();
                        let op = Operation::start(&format!("Calling the tool {tool_id}"));
                        let tool_fetcher = self.router.get_tool(tool_call.id);
                        let tool_link = tool_fetcher.await?;
                        let tool_response = tool_link.call_tool(tool_call.args).await?;
                        let message = Message::from(tool_response);
                        extra_messages.push(message);
                        op.end(&format!("Tool call {tool_id} completed"));
                    }
                } else {
                    extra_messages.push(message.into());
                }
            }

            if one_more_step {
                request.messages.extend(extra_messages.drain(..));
            } else {
                break;
            }
        }

        let response = ChatResponse {
            messages: extra_messages,
        };
        Ok(response)
    }
}
