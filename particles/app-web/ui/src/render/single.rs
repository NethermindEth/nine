use super::projection::{Projection, ProjectionStream};
use crb::core::watch::Ref;
use derive_more::{Deref, From};
use futures::{Stream, StreamExt};
use ui9::names::Fqn;
use ui9_dui::{State, Sub, SubEvent, Subscriber};
use yew::Properties;

pub struct SingleFlow<F: Subscriber> {
    sub: Sub<F>,
    state: Option<State<F>>,
    lost: bool,
}

impl<F: Subscriber> Projection for SingleFlow<F> {
    type Message = Msg<F>;
    type Properties = Props;
    type State<'a> = SingleState<'a, F>;

    fn create(props: &Self::Properties) -> Self {
        let fqn = props.fqn.clone();
        let sub = Sub::<F>::local(fqn);
        Self {
            sub,
            state: None,
            lost: false,
        }
    }

    fn streams(&mut self) -> Vec<ProjectionStream<Self::Message>> {
        let stream = self.sub.events().unwrap().map(Msg::Event).boxed();
        vec![stream]
    }

    // TODO: Provide a reference to a component
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Event(sub_event) => {
                // self.component.on_sub(&sub_event);
                // Events processing
                match sub_event {
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
        }
        true
    }

    fn state(&self) -> Option<Self::State<'_>> {
        let state = self.state.as_ref()?.borrow();
        Some(SingleState {
            state,
            lost: self.lost,
        })
    }
}

#[derive(From)]
pub enum Msg<F: Subscriber> {
    Event(SubEvent<F>),
}

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub fqn: Fqn,
}

#[derive(Deref)]
pub struct SingleState<'a, F> {
    #[deref]
    state: Ref<'a, F>,
    lost: bool,
}
