use super::projection::{Projection, ProjectionStream, StateTracker, StateView};
use derive_more::{Deref, From};
use futures::{Stream, StreamExt};
use ui9::names::Fqn;
use ui9_dui::{SubEvent, Subscriber};
use yew::Properties;

pub struct SingleFlow<F: Subscriber> {
    tracker: StateTracker<F>,
}

pub enum Msg<F: Subscriber> {
    Event(SubEvent<F>),
}

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub fqn: Fqn,
}

#[derive(Deref, From)]
pub struct SingleState<'a, F> {
    pub view: StateView<'a, F>,
}

impl<F: Subscriber> Projection for SingleFlow<F> {
    type Message = Msg<F>;
    type Properties = Props;
    type State<'a> = SingleState<'a, F>;

    fn create(props: &Self::Properties) -> Self {
        let fqn = props.fqn.clone();
        Self {
            tracker: StateTracker::new(fqn),
        }
    }

    fn streams(&mut self) -> Vec<ProjectionStream<Self::Message>> {
        let stream = self.tracker.sub.events().unwrap().map(Msg::Event).boxed();
        vec![stream]
    }

    // TODO: Provide a reference to a component
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Event(sub_event) => {
                // self.component.on_sub(&sub_event);
                // Events processing
                self.tracker.update(sub_event);
            }
        }
        true
    }

    fn state(&self) -> Option<Self::State<'_>> {
        self.tracker.state_view().map(SingleState::from)
    }
}
