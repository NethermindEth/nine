use super::types::{CallInfo, ToolCall, ToolId, ToolInfo, ToolMeta, ToolResult};
use super::{ReasoningRouter, RouterLink};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, MessageFor};
use crb::send::{Recipient, Sender};
use crb::superagent::{
    Fetcher, InteractExt, Interaction, Interplay, OnRequest, Request, Responder,
};
use derive_more::{Deref, DerefMut};
use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::{de::DeserializeOwned, Serialize};
use std::any::type_name;
use std::marker::PhantomData;
use std::sync::Arc;
use ui9::names::Fqn;
use ui9_dui::Operation;

pub struct CallMeta {
    pub info: CallInfo,
    pub chat: Fqn,
}

pub trait Prompt: ToolData {
    type Output: ToolData;

    fn description() -> &'static str;
}

pub trait ToolData: JsonSchema + Serialize + DeserializeOwned + Send + 'static {}

impl<T> ToolData for T where T: JsonSchema + Serialize + DeserializeOwned + Send + 'static {}

#[async_trait]
pub trait Tool<P>
where
    Self: Agent,
    P: Prompt,
{
    fn name(&self) -> String {
        let mut name = String::new();
        let full_name = type_name::<P>()
            .to_lowercase()
            .replace('<', "_")
            .replace('>', "");
        let items: Vec<_> = full_name.split("::").collect();
        if let Some(first) = items.first() {
            name.push_str(first);
        }
        if let Some(last) = items.last() {
            if !name.is_empty() {
                name.push('_');
            }
            name.push_str(last);
        }
        name
    }

    fn description(&self) -> Option<String> {
        Some(P::description().into())
    }

    fn parameters(&self) -> Option<RootSchema> {
        let schema = schema_for!(P);
        Some(schema)
    }

    async fn handle_request(
        &mut self,
        // TODO: Use a custom wrapper for `Interplay`
        msg: Interaction<CallTool>,
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        let CallTool { request, chat } = msg.interplay.request;
        let info = request.info.clone();
        let meta = CallMeta { info, chat };
        match serde_json::from_value(request.args) {
            Ok(request) => {
                self.handle_response(meta, request, msg.interplay.responder, ctx)
                    .await
            }
            Err(err) => msg.interplay.responder.send_result(Err(err.into())),
        }
    }

    async fn handle_response(
        &mut self,
        meta: CallMeta,
        msg: P,
        responder: Responder<ToolResult>,
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        let info = meta.info.clone();
        let output = self.call_tool_meta(msg, meta, ctx).await?;
        let value = serde_json::to_value(&output)?;
        let res = ToolResult { info, value };
        responder.send_result(Ok(res))
    }

    async fn call_tool_meta(
        &mut self,
        input: P,
        _meta: CallMeta,
        _ctx: &mut Context<Self>,
    ) -> Result<P::Output> {
        self.call_tool(input).await
    }

    async fn call_tool(&mut self, _input: P) -> Result<P::Output> {
        Err(anyhow!("Not implemented"))
    }
}

#[derive(Deref, DerefMut, Clone)]
pub struct ToolLink {
    address: Arc<dyn ToolAddress>,
}

pub trait ToolAddress: Sync + Send {
    fn call_tool(&self, chat: Fqn, request: ToolCall) -> Fetcher<ToolResult>;
}

struct ToolLinkRaw<P> {
    recipient: Recipient<CallToolTyped<P>>,
}

impl<P> ToolAddress for ToolLinkRaw<P>
where
    P: Prompt,
{
    fn call_tool(&self, chat: Fqn, request: ToolCall) -> Fetcher<ToolResult> {
        let request = CallTool { request, chat };
        let (interplay, fetcher) = Interplay::new_pair(request);
        let interaction = Interaction { interplay };
        let msg = CallToolTyped {
            _type: PhantomData::<P>,
            interaction,
        };
        let res = self.recipient.send(msg);
        fetcher.grasp(res)
    }
}

impl RouterLink {
    pub async fn add_tool<A, P>(&mut self, addr: Address<A>, meta: ToolMeta) -> Result<ToolId>
    where
        A: Tool<P>,
        P: Prompt,
    {
        let raw_link = ToolLinkRaw {
            recipient: addr.sender(),
        };
        let link = ToolLink {
            address: Arc::new(raw_link),
        };
        let msg = AddTool { link, meta };
        let response = self.address.interact(msg).await?;
        Ok(response.info.id.clone())
    }

    pub fn get_tools(&mut self) -> Fetcher<Vec<ToolInfo>> {
        self.interact(GetTools)
    }

    pub fn get_tool(&mut self, id: ToolId) -> Fetcher<ToolLink> {
        self.interact(GetTool { id })
    }
}

pub struct AddTool {
    link: ToolLink,
    meta: ToolMeta,
}

pub struct ToolAdded {
    pub info: ToolInfo,
}

impl Request for AddTool {
    type Response = ToolAdded;
}

pub struct ToolRecord {
    pub link: ToolLink,
    pub info: ToolInfo,
}

#[async_trait]
impl OnRequest<AddTool> for ReasoningRouter {
    async fn on_request(&mut self, msg: AddTool, _ctx: &mut Context<Self>) -> Result<ToolAdded> {
        let id = ToolId::from(format!("{}_{}", msg.meta.name, self.tools.len()));

        let op = Operation::start(&format!("Add tool {id}"));

        let info = ToolInfo {
            id: id.clone(),
            meta: msg.meta,
        };
        let record = ToolRecord {
            link: msg.link,
            info: info.clone(),
        };
        self.tools.insert(id, record);
        self.tools_pub.add_tool(&info);

        op.end();

        Ok(ToolAdded { info })
    }
}

pub struct CallTool {
    pub request: ToolCall,
    // TODO: Consider to use a typed link here
    pub chat: Fqn,
}

impl Request for CallTool {
    type Response = ToolResult;
}

struct CallToolTyped<P> {
    _type: PhantomData<P>,
    interaction: Interaction<CallTool>,
}

#[async_trait]
impl<A, P> MessageFor<A> for CallToolTyped<P>
where
    A: Tool<P>,
    P: Prompt,
{
    async fn handle(self: Box<Self>, agent: &mut A, ctx: &mut Context<A>) -> Result<()> {
        agent.handle_request(self.interaction, ctx).await
    }
}

struct GetTools;

impl Request for GetTools {
    type Response = Vec<ToolInfo>;
}

#[async_trait]
impl OnRequest<GetTools> for ReasoningRouter {
    async fn on_request(&mut self, _: GetTools, _ctx: &mut Context<Self>) -> Result<Vec<ToolInfo>> {
        // TODO: Keep info in `Arc`s
        Ok(self
            .tools
            .values()
            .map(|record| record.info.clone())
            .collect())
    }
}

struct GetTool {
    id: ToolId,
}

impl Request for GetTool {
    type Response = ToolLink;
}

#[async_trait]
impl OnRequest<GetTool> for ReasoningRouter {
    async fn on_request(&mut self, msg: GetTool, _ctx: &mut Context<Self>) -> Result<ToolLink> {
        self.tools
            .get(&msg.id)
            .map(|record| record.link.clone())
            .ok_or_else(|| anyhow!("Tool {} is not available.", msg.id))
    }
}
