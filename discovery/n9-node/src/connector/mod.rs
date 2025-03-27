#[cfg(feature = "std")]
pub mod swarm_std;
#[cfg(feature = "std")]
use swarm_std as swarm_impl;

#[cfg(feature = "web")]
pub mod swarm_web;
#[cfg(feature = "web")]
use swarm_web as swarm_impl;

mod behaviour;
mod keypair;

use anyhow::Result;
use async_trait::async_trait;
use behaviour::{NineBehaviour, NineBehaviourEvent};
use crb::agent::{Address, Agent, AgentContext, Context, DoAsync, ManagedContext, Next, OnEvent};
use crb::core::Slot;
use crb::superagent::{Fetcher, InteractExt, Interval, OnRequest, Request, StreamSession, Tick};
use derive_more::{Deref, DerefMut, From};
use futures::stream::StreamExt;
pub use keypair::Key;
use libp2p::{gossipsub, swarm::SwarmEvent, Multiaddr, Swarm};
use libp2p_request_response::{self as request_response};
use libp2p_stream as stream;
use tokio::select;

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
    key: Key,
    swarm: Slot<Swarm<NineBehaviour>>,
    topic: gossipsub::IdentTopic,
    interval: Interval,
}

impl Connector {
    pub fn new(key: Key) -> Self {
        Self {
            key,
            swarm: Slot::empty(),
            topic: gossipsub::IdentTopic::new("n9"),
            interval: Interval::new(),
        }
    }
}

#[async_trait]
impl Agent for Connector {
    type Context = StreamSession<Self>;

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
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        self.interval.set_interval_ms(5_000)?;
        ctx.consume(self.interval.events()?);

        let mut swarm = swarm_impl::swarm(&self.key).await?;
        swarm.behaviour_mut().gossipsub.subscribe(&self.topic)?;

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
        event: SwarmEvent<NineBehaviourEvent>,
        ctx: &mut Context<Self>,
    ) -> Result<()> {
        match event {
            SwarmEvent::Behaviour(event) => match event {
                #[cfg(feature = "mdns")]
                NineBehaviourEvent::Mdns(event) => {
                    OnEvent::handle(self, event, ctx).await?;
                }
                NineBehaviourEvent::Gossipsub(event) => {
                    OnEvent::handle(self, event, ctx).await?;
                }
                NineBehaviourEvent::RequestResponse(event) => {
                    OnEvent::handle(self, event, ctx).await?;
                }
                NineBehaviourEvent::Stream(()) => {
                    log::info!("Start streaming");
                }
            },
            SwarmEvent::NewListenAddr { address, .. } => {
                log::info!("Local node is listening on {address}");
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                log::debug!("Connection to {peer_id} has established");
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                log::debug!("Connection to {peer_id} has closed");
            }
            SwarmEvent::IncomingConnection { .. } => {}
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
                    log::trace!("Nine node discovered: {peer_id}");
                    swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                }
            }
            Expired(list) => {
                for (peer_id, _multiaddr) in list {
                    log::trace!("Nine node exipred: {peer_id}");
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

#[async_trait]
impl OnEvent<Tick> for Connector {
    async fn handle(&mut self, _event: Tick, _ctx: &mut Context<Self>) -> Result<()> {
        let topic = self.topic.clone();
        let swarm = self.swarm.get_mut()?;
        swarm
            .behaviour_mut()
            .gossipsub
            .publish(topic, "state".as_bytes())?;
        Ok(())
    }
}
