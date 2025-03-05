use super::projection::{Projection, ProjectionStream, StateTracker, StateView};
use futures::StreamExt;
use ui9::names::Fqn;
use ui9_dui::{Sub, SubEvent, Subscriber};
use yew::Properties;

pub struct DoubleFlow<F: Subscriber, S: Subscriber> {
    first: StateTracker<F>,
    second: StateTracker<S>,
}

pub enum Msg<F: Subscriber, S: Subscriber> {
    First(SubEvent<F>),
    Second(SubEvent<S>),
}

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub first: Fqn,
    pub second: Fqn,
}

pub struct DoubleState<'a, F, S> {
    pub first: StateView<'a, F>,
    pub second: StateView<'a, S>,
}

impl<F: Subscriber, S: Subscriber> Projection for DoubleFlow<F, S> {
    type Message = Msg<F, S>;
    type Properties = Props;
    type State<'a> = DoubleState<'a, F, S>;

    fn create(props: &Self::Properties) -> Self {
        let first = props.first.clone();
        let second = props.second.clone();
        Self {
            first: StateTracker::new(first),
            second: StateTracker::new(second),
        }
    }

    fn streams(&mut self) -> Vec<ProjectionStream<Self::Message>> {
        let first = self.first.sub.events().unwrap().map(Msg::First).boxed();
        let second = self.second.sub.events().unwrap().map(Msg::Second).boxed();
        vec![first, second]
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::First(sub_event) => {
                self.first.update(sub_event);
            }
            Msg::Second(sub_event) => {
                self.second.update(sub_event);
            }
        }
        true
    }

    fn state(&self) -> Option<Self::State<'_>> {
        None
    }
}
