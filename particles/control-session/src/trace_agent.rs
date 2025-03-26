use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::superagent::StreamSession;
use n9_core::unroller::{
    Operation, OperationDetails, OperationId, OperationInfo, UnrollerAction, UnrollerFlow,
};
use std::collections::HashMap;
use ui9::names::Fqn;
use ui9_dui::{Act, Pub};

pub struct TraceAgent {
    tracer: Pub<UnrollerFlow>,
    operations: HashMap<OperationId, Operation>,
}

impl TraceAgent {
    pub fn new(fqn: Fqn) -> Self {
        Self {
            tracer: Pub::new(fqn),
            operations: HashMap::new(),
        }
    }
}

impl Agent for TraceAgent {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for TraceAgent {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        ctx.consume(self.tracer.actions()?);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<UnrollerFlow>> for TraceAgent {
    async fn handle(&mut self, msg: Act<UnrollerFlow>, ctx: &mut Context<Self>) -> Result<()> {
        match msg.action {
            UnrollerAction::Operation(operation) => {
                let id = OperationId::new_v4();
                let info = OperationInfo {
                    id,
                    timestamp: Utc::now().naive_utc(),
                    task: self.reason(&operation),
                    op_type: operation.get_type(),
                };
                self.operations.insert(id, operation);
                self.tracer.operation(info);
                self.load(id);
            }
            UnrollerAction::Done => {
                self.tracer.complete();
            }
            UnrollerAction::Show(id) => {
                self.load(id);
            }
        }
        Ok(())
    }
}

impl TraceAgent {
    fn load(&mut self, id: OperationId) {
        let operation = self
            .operations
            .get(&id)
            .cloned()
            .map(|operation| OperationDetails { id, operation });
        self.tracer.show(operation);
    }

    fn reason(&self, op: &Operation) -> String {
        match op {
            Operation::Request(request) => "Sending a request".into(),
            Operation::Response(response) => "Fetching a response".into(),
            Operation::ToolCall(call) => format!("Calling a tool: {}", call.info.tool_id),
            Operation::ToolResult(response) => {
                format!("Tool call result: {}", response.info.tool_id)
            }
        }
    }
}
