use crb::agent::{Agent, AgentSession};

pub struct LiquidTool;

impl Agent for LiquidTool {
    type Context = AgentSession<Self>;
}
