use super::projection::{Projection, ProjectionStream, StateTracker, StateView};
use derive_more::{Deref, DerefMut};
use futures::StreamExt;
use ui9_dui::{SubEvent, Subscriber};
use ui9_net::FqnLink;
use yew::Properties;

pub struct Flow<F: Subscriber, S: Subscriber> {
    pub first: StateTracker<F>,
    pub second: StateTracker<S>,
}

pub enum Msg<F: Subscriber, S: Subscriber> {
    First(SubEvent<F>),
    Second(SubEvent<S>),
}

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub first: FqnLink,
    pub second: FqnLink,
}

#[derive(Deref, DerefMut)]
pub struct State<'a, F, S> {
    #[deref]
    #[deref_mut]
    pub first: StateView<'a, F>,
    pub second: StateView<'a, S>,
}

impl<F: Subscriber, S: Subscriber> Projection for Flow<F, S> {
    type Message = Msg<F, S>;
    type Properties = Props;
    type State<'a> = State<'a, F, S>;

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
        let first = self.first.state_view()?;
        let second = self.second.state_view()?;
        Some(State { first, second })
    }
}
