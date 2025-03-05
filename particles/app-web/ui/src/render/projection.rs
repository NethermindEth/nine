use futures::Stream;
use std::pin::Pin;
use yew::Properties;

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

    fn create(props: &Self::Properties) -> Self {
        ()
    }

    fn streams(&mut self) -> Vec<ProjectionStream<Self::Message>> {
        Vec::new()
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        true
    }

    fn state(&self) -> Option<Self::State<'_>> {
        Some(())
    }
}
