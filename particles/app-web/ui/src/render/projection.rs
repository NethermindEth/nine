use crb::core::watch::Ref;
use derive_more::Deref;
use futures::Stream;
use libp2p::PeerId;
use std::pin::Pin;
use ui9::names::Fqn;
use ui9_dui::{State, Sub, SubEvent, Subscriber};
use ui9_net::RemoteExt;
use yew::Properties;

#[derive(Deref)]
pub struct StateTracker<F: Subscriber> {
    #[deref]
    pub sub: Sub<F>,
    pub state: Option<State<F>>,
    pub lost: bool,
}

#[derive(Deref)]
pub struct StateView<'a, F> {
    #[deref]
    pub state: Ref<'a, F>,
    pub lost: bool,
}

impl<F: Subscriber> StateTracker<F> {
    pub fn new(fqn: Fqn, peer: Option<PeerId>) -> Self {
        let sub = {
            if let Some(peer) = peer {
                Sub::remote(peer, fqn)
            } else {
                Sub::local(fqn)
            }
        };
        Self {
            sub,
            state: None,
            lost: false,
        }
    }

    pub fn update(&mut self, event: SubEvent<F>) {
        match event {
            SubEvent::State(state) => {
                self.state = Some(state);
                self.lost = false;
            }
            SubEvent::Event(_event) => {}
            SubEvent::Lost => {
                self.lost = true;
            }
        }
    }

    pub fn state_view(&self) -> Option<StateView<'_, F>> {
        let state = self.state.as_ref()?.borrow();
        Some(StateView {
            state,
            lost: self.lost,
        })
    }
}

pub type ProjectionStream<M> = Pin<Box<dyn Stream<Item = M>>>;

pub trait Projection {
    type Message;
    type Properties: Properties;
    type State<'a>
    where
        Self: 'a;

    fn create(props: &Self::Properties) -> Self;

    fn streams(&mut self) -> Vec<ProjectionStream<Self::Message>>;

    fn update(&mut self, msg: Self::Message) -> bool;

    fn state(&self) -> Option<Self::State<'_>>;
}

impl Projection for () {
    type Message = ();
    type Properties = ();
    type State<'a> = ();

    fn create(_props: &Self::Properties) -> Self {
        ()
    }

    fn streams(&mut self) -> Vec<ProjectionStream<Self::Message>> {
        Vec::new()
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn state(&self) -> Option<Self::State<'_>> {
        Some(())
    }
}
