use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use crb::agent::{Agent, Context, DoAsync, Next, OnEvent};
use crb::core::uuid::Uuid;
use crb::superagent::StreamSession;
use n9_core::chain::{Operation, OperationInfo, ReasoningAction, ReasoningFlow};
use ui9::names::Fqn;
use ui9_dui::{Act, Pub};

pub struct TraceAgent {
    tracer: Pub<ReasoningFlow>,
    operations: Vec<Operation>,
}

impl TraceAgent {
    pub fn new(fqn: Fqn) -> Self {
        Self {
            tracer: Pub::new(fqn),
            operations: Vec::new(),
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
impl OnEvent<Act<ReasoningFlow>> for TraceAgent {
    async fn handle(&mut self, msg: Act<ReasoningFlow>, ctx: &mut Context<Self>) -> Result<()> {
        match msg.action {
            ReasoningAction::Operation(operation) => {
                let info = OperationInfo {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now().naive_utc(),
                    // TODO: Make a better reason
                    task: "Reason".into(),
                };
                self.operations.push(operation);
                self.tracer.operation(info);
            }
        }
        Ok(())
    }
}
