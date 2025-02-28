use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, DoAsync, Next, Standalone};

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
impl DoAsync<Bootstrap> for Frontend {}
