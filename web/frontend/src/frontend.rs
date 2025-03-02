use crate::web_app::WebApp;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, DoAsync, Next, Standalone};
use ui9_dui::Hub;

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
        Hub::activate().await?;
        yew::Renderer::<WebApp>::new().render();
        Ok(Next::events())
    }
}
