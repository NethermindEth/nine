use crate::flow::chat_control::ChatControl;
use crate::flow::session_control::{SessionControl, SessionControlAction, SessionKey};
use anyhow::{Error, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, OnEvent};
use crb::superagent::{StreamSession, Supervisor};
use n9_core::{Particle, SubstanceLinks};
use std::collections::HashMap;
use ui9_dui::{Act, Operation, Pub};

struct SessionRecord {
    chat_control: Pub<ChatControl>,
}

/*
impl SessionRecord {
    pub fn new() -> Self {
        Self {
            chat_control: Pub::new(),
        }
    }
}
*/

pub struct SessionParticle {
    substance: SubstanceLinks,
    session_control: Pub<SessionControl>,
    sessions: HashMap<SessionKey, SessionRecord>,
}

impl Particle for SessionParticle {
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            session_control: Pub::unified(),
            sessions: HashMap::new(),
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
        // TODO: Try to restore sessions from a persistent layer
        let actions = self.session_control.actions()?;
        ctx.consume(actions);
        Ok(Next::events())
    }
}

#[async_trait]
impl OnEvent<Act<SessionControl>> for SessionParticle {
    async fn handle(&mut self, msg: Act<SessionControl>, ctx: &mut Context<Self>) -> Result<()> {
        match msg.action {
            SessionControlAction::Create { key } => {
                if !self.sessions.contains_key(&key) {
                    /*
                    let session = SessionRecord::new();
                    self.sessions.insert(key, session);
                    */
                }
            }
        }
        Ok(())
    }
}
