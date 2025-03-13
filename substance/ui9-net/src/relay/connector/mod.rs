#[cfg(feature = "std")]
pub mod swarm_std;
#[cfg(feature = "std")]
use swarm_std as swarm_impl;

#[cfg(feature = "web")]
pub mod swarm_web;
#[cfg(feature = "web")]
use swarm_web as swarm_impl;

pub mod behaviour;

use crate::relay::keypair::Key;
use crate::tracers::peer::Peer;
use anyhow::Result;
use async_trait::async_trait;
use behaviour::{Ui9Behaviour, Ui9BehaviourEvent};
use crb::agent::{
    Address, Agent, AgentContext, AgentSession, Context, DoAsync, ManagedContext, Next, OnEvent,
};
use crb::core::Slot;
use crb::superagent::{Fetcher, InteractExt, OnRequest, Request};
use derive_more::{Deref, DerefMut, From};
use futures::stream::StreamExt;
use libp2p::{gossipsub, swarm::SwarmEvent, Multiaddr, Swarm};
use libp2p_request_response::{self as request_response};
use libp2p_stream as stream;
use tokio::select;
use ui9_dui::Pub;

#[cfg(feature = "mdns")]
use libp2p::mdns;

#[derive(Deref, DerefMut, From, Clone)]
pub struct ConnectorLink {
    connector: Address<Connector>,
}

impl ConnectorLink {
    pub fn get_control(&self) -> Fetcher<stream::Control> {
        let msg = GetControl;
        self.connector.interact(msg)
    }

    pub fn bootstrap(&self, address: Multiaddr) -> Result<()> {
        self.connector.event(Bootstrap { address })
    }
}

pub struct Connector {
    swarm: Slot<Swarm<Ui9Behaviour>>,
    peer_tracer: Pub<Peer>,
}

impl Connector {
    pub fn new() -> Self {
        Self {
            swarm: Slot::empty(),
            peer_tracer: Pub::unified(),
        }
    }
}

#[async_trait]
impl Agent for Connector {
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }

    async fn event(&mut self, ctx: &mut Context<Self>) -> Result<()> {
        let swarm = self.swarm.get_mut()?;
        select! {
            envelope = ctx.next_envelope() => {
                if let Some(envelope) = envelope {
                    envelope.handle(self, ctx).await?;
                } else {
                    ctx.stop();
                }
            }
            event = swarm.select_next_some() => {
                self.route_swarm_event(event, ctx).await?;
            }
        }
        Ok(())
    }
}

pub struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Connector {
    async fn handle(&mut self, _: Initialize, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let key = Key::generate();
        let mut swarm = swarm_impl::swarm(&key).await?;

        let topic = gossipsub::IdentTopic::new("n9");
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        #[cfg(feature = "tcp")]
        {
            // swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
            swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
            swarm.listen_on("/ip4/127.0.0.1/tcp/2020/ws".parse()?)?;
        }

        self.swarm.fill(swarm)?;
        Ok(Next::events())
    }
}

impl Connector {
    async fn route_swarm_event(
        &mut self,
        event: SwarmEvent<Ui9BehaviourEvent>,
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        match event {
            SwarmEvent::Behaviour(event) => match event {
                #[cfg(feature = "mdns")]
                Ui9BehaviourEvent::Mdns(event) => {
                    OnEvent::handle(self, event, ctx).await?;
                }
                Ui9BehaviourEvent::Gossipsub(event) => {
                    OnEvent::handle(self, event, ctx).await?;
                }
                Ui9BehaviourEvent::RequestResponse(event) => {
                    OnEvent::handle(self, event, ctx).await?;
                }
                Ui9BehaviourEvent::Stream(()) => {
                    log::info!("Start streaming");
                }
            },
            SwarmEvent::NewListenAddr { address, .. } => {
                log::info!("Local node is listening on {address}");
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                self.peer_tracer.add_peer(peer_id);
                log::debug!("Connection to {peer_id} has established");
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                self.peer_tracer.del_peer(peer_id);
                log::debug!("Connection to {peer_id} has closed");
            }
            SwarmEvent::IncomingConnection { connection_id, .. } => {}
            other => {
                log::warn!("Not handeled p2p event: {other:?}");
            }
        }
        Ok(())
    }
}

#[cfg(feature = "mdns")]
#[async_trait]
impl OnEvent<mdns::Event> for Connector {
    async fn handle(&mut self, event: mdns::Event, _ctx: &mut Context<Self>) -> Result<()> {
        use mdns::Event::*;
        let swarm = self.swarm.get_mut()?;
        match event {
            Discovered(list) => {
                for (peer_id, _multiaddr) in list {
                    log::trace!("UI9 node discovered: {peer_id}");
                    swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                }
            }
            Expired(list) => {
                for (peer_id, _multiaddr) in list {
                    log::trace!("UI9 node exipred: {peer_id}");
                    swarm
                        .behaviour_mut()
                        .gossipsub
                        .remove_explicit_peer(&peer_id);
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl OnEvent<gossipsub::Event> for Connector {
    async fn handle(&mut self, event: gossipsub::Event, _ctx: &mut Context<Self>) -> Result<()> {
        use gossipsub::Event::*;
        if let Message {
            propagation_source,
            message_id,
            message,
        } = event
        {
            log::trace!(
                "Got message: '{}' with id: {message_id} from peer: {propagation_source}",
                String::from_utf8_lossy(&message.data),
            );
        }
        Ok(())
    }
}

#[async_trait]
impl OnEvent<request_response::Event<(), ()>> for Connector {
    async fn handle(
        &mut self,
        event: request_response::Event<(), ()>,
        _ctx: &mut Context<Self>,
    ) -> Result<()> {
        use request_response::{Event, Message};
        match event {
            Event::Message { message, .. } => match message {
                Message::Request { request, .. } => {
                    log::warn!("Not handeled request event: {request:?}");
                }
                Message::Response { response, .. } => {
                    log::warn!("Not handeled response event: {response:?}");
                }
            },
            other => {
                log::warn!("Not handeled request_reponse event: {other:?}");
            }
        }
        Ok(())
    }
}

pub struct GetControl;

impl Request for GetControl {
    type Response = stream::Control;
}

#[async_trait]
impl OnRequest<GetControl> for Connector {
    async fn on_request(
        &mut self,
        _req: GetControl,
        _ctx: &mut Context<Self>,
    ) -> Result<stream::Control> {
        let swarm = self.swarm.get_mut()?;
        let control = swarm.behaviour_mut().stream.new_control();
        Ok(control)
    }
}

pub struct Bootstrap {
    address: Multiaddr,
}

#[async_trait]
impl OnEvent<Bootstrap> for Connector {
    async fn handle(&mut self, event: Bootstrap, _ctx: &mut Context<Self>) -> Result<()> {
        let addr = event.address;
        log::info!("Dialing {addr}");
        self.swarm.get_mut()?.dial(addr)?;
        Ok(())
    }
}
