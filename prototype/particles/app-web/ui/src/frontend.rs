use crate::web_app::WebApp;
use crate::widgets::dashboard::DashboardWorker;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, Standalone};
use crb::superagent::{Supervisor, SupervisorSession};
use ui9_mesh::Mesh;
use ui9_net::MeshNode;

pub struct Frontend;

impl Frontend {
    pub fn new() -> Self {
        Self
    }
}

impl Standalone for Frontend {}

impl Supervisor for Frontend {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for Frontend {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Bootstrap)
    }
}

struct Bootstrap;

#[async_trait]
impl DoAsync<Bootstrap> for Frontend {
    async fn handle(&mut self, _: Bootstrap, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        Mesh::activate().await?;
        let worker = DashboardWorker::new();
        ctx.spawn_agent(worker, ());
        let addr = crate::net::server_multiaddr()?;
        MeshNode::link()?.connector.bootstrap(addr)?;
        yew::Renderer::<WebApp>::new().render();
        Ok(Next::events())
    }
}
