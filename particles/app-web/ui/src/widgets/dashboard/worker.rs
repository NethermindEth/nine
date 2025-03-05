use super::flow::Dashboard;
use crb::agent::{Agent, AgentSession};
use ui9_dui::Pub;

pub struct DashboardWorker {
    state: Pub<Dashboard>,
}

impl DashboardWorker {
    pub fn new() -> Self {
        Self {
            state: Pub::unified(),
        }
    }
}

impl Agent for DashboardWorker {
    type Context = AgentSession<Self>;
}
