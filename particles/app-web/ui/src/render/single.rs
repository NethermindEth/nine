use super::projection::{Projection, ProjectionStream, StateTracker, StateView};
use derive_more::{Deref, DerefMut};
use futures::StreamExt;
use ui9_dui::{SubEvent, Subscriber};
use ui9_net::FqnLink;
use yew::Properties;

#[derive(Deref, DerefMut)]
pub struct Flow<F: Subscriber> {
    pub tracker: StateTracker<F>,
}

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub link: FqnLink,
}

pub type State<'a, F> = StateView<'a, F>;

impl<F: Subscriber> Projection for Flow<F> {
    type Message = SubEvent<F>;
    type Properties = Props;
    type State<'a> = StateView<'a, F>;

    fn create(props: &Self::Properties) -> Self {
        let link = props.link.clone();
        Self {
            tracker: StateTracker::new(link),
        }
    }

    fn streams(&mut self) -> Vec<ProjectionStream<Self::Message>> {
        let stream = self.tracker.sub.events().unwrap().boxed();
        vec![stream]
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.tracker.update(msg);
        true
    }

    fn state(&self) -> Option<Self::State<'_>> {
        self.tracker.state_view()
    }
}
