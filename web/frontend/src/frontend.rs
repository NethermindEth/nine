use crate::web_app::WebApp;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, DoAsync, Next, Standalone};
use ui9_net::MeshNode;
use ui9_mesh::Mesh;

pub struct Frontend;

impl Frontend {
    pub fn new() -> Self {
        Self
    }
}

impl Standalone for Frontend {}

impl Agent for Frontend {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Bootstrap)
    }
}

struct Bootstrap;

#[async_trait]
impl DoAsync<Bootstrap> for Frontend {
    async fn once(&mut self, _: &mut Bootstrap) -> Result<Next<Self>> {
        Mesh::activate().await?;
        let addr = crate::net::server_multiaddr()?;
        MeshNode::link()?.connector.bootstrap(addr)?;
        yew::Renderer::<WebApp>::new().render();
        Ok(Next::events())
    }
}
