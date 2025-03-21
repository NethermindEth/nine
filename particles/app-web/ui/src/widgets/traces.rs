use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_core::chain::{Operation, OperationDetails, OperationId, OperationInfo, ReasoningFlow};
use n9_core::{Message, ToolCall, ToolResult, ToolingChatRequest, ToolingChatResponse};
use yew::{html, Html};

pub type TracesWidget = SubWidget<TracesComponent>;

pub struct TracesComponent {}

#[derive(Clone)]
pub enum Msg {
    Load(OperationId),
}

impl SubComponent for TracesComponent {
    type Projection = single::Flow<ReasoningFlow>;
    type Message = Msg;

    fn create() -> Self {
        Self {}
    }

    fn update(
        &mut self,
        msg: Self::Message,
        pro: &mut Self::Projection,
        _ctx: &SubContext<Self>,
    ) -> bool {
        match msg {
            Msg::Load(id) => {
                // TODO: Rename to `load`
                pro.show(id);
            }
        }
        true
    }

    fn render(&self, state: single::State<ReasoningFlow>, ctx: &SubContext<Self>) -> Option<Html> {
        html! {
            <div class="widget-traces">
                <div class="widget-traces-list">
                    { for state.operations.iter().map(|op| self.render_operation(op, ctx)) }
                </div>
                <div class="widget-traces-details">
                    { state.operation.as_ref().map(|op| self.render_details(op, ctx)) }
                </div>
            </div>
        }
        .into()
    }
}

impl TracesComponent {
    fn render_operation(&self, op: &OperationInfo, ctx: &SubContext<Self>) -> Html {
        let onclick = ctx.event(Msg::Load(op.id));
        html! {
            <div {onclick} class="widget-traces-list-item">
                { &op.task }
            </div>
        }
    }

    fn render_details(&self, details: &OperationDetails, ctx: &SubContext<Self>) -> Html {
        match &details.operation {
            Operation::Request(value) => self.render_request(value, ctx),
            Operation::Response(value) => self.render_response(value, ctx),
            Operation::ToolCall(value) => self.render_tool_call(value, ctx),
            Operation::ToolResult(value) => self.render_tool_result(value, ctx),
        }
    }

    fn render_request(&self, request: &ToolingChatRequest, ctx: &SubContext<Self>) -> Html {
        html! {
            <div class="widget-traces-request">
                { for request.messages.iter().map(|msg| self.render_message(msg, ctx)) }
            </div>
        }
    }

    fn render_response(&self, response: &ToolingChatResponse, ctx: &SubContext<Self>) -> Html {
        html! {}
    }

    fn render_tool_call(&self, tool_call: &ToolCall, ctx: &SubContext<Self>) -> Html {
        html! {}
    }

    fn render_tool_result(&self, tool_result: &ToolResult, ctx: &SubContext<Self>) -> Html {
        html! {}
    }

    fn render_message(&self, message: &Message, ctx: &SubContext<Self>) -> Html {
        html! {
            <div class="widget-traces-message">
                <div class="widget-traces-message-content">
                    { &message.content }
                </div>
                <div class="widget-traces-message-role">
                    { message.role.to_string() }
                </div>
            </div>
        }
    }
}
