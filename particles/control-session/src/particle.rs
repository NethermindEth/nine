use crate::chat_loop::ChatControlLoop;
use crate::flow::chat_control::ChatControl;
use crate::flow::session_control::{SessionControl, SessionControlAction, SessionInfo, SessionKey};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use crb::agent::{Address, Agent, AgentSession, Context, DoAsync, Equip, Next, OnEvent};
use crb::superagent::{StreamSession, Supervisor, SupervisorSession};
use n9_core::{Particle, SubstanceLinks};
use std::collections::HashMap;
use ui9_dui::{Act, Pub};

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
    type BasedOn = StreamSession<Self>;
    type GroupBy = SessionKey;
}

impl Agent for SessionParticle {
    type Context = SupervisorSession<Self>;

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
                    let session = self.spawn_session(key.clone(), ctx);
                    self.sessions.insert(key.clone(), session);

                    let utc_now = Utc::now();
                    let info = SessionInfo {
                        created: utc_now.naive_utc(),
                        title: None,
                    };
                    self.session_control.add(key, info);
                }
            }
        }
        Ok(())
    }
}

struct SessionRecord {
    address: Address<ChatControlLoop>,
}

impl SessionParticle {
    fn spawn_session(&mut self, key: SessionKey, ctx: &mut Context<Self>) -> SessionRecord {
        let router = self.substance.router.clone();
        let chat_control = ChatControlLoop::new(router, key.clone());
        let address = ctx.spawn_agent(chat_control, key).equip();
        let record = SessionRecord { address };
        record
    }
}
