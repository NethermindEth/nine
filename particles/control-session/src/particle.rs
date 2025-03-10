use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Next, DoAsync, Context, Agent, AgentSession};
use crb::superagent::{Supervisor, StreamSession};
use n9_core::{SubstanceLinks, Particle};

pub struct SessionParticle {
    substance: SubstanceLinks,
}

impl Particle for SessionParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
        }
    }
}

impl Supervisor for SessionParticle {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for SessionParticle {
    type Context = StreamSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for SessionParticle {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        // ctx.consume(self.chat.actions()?);
        Ok(Next::events())
    }
}

