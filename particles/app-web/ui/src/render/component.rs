use ui9_dui::{SubEvent, Subscriber};
use yew::Html;

pub trait SubComponent: 'static {
    type Flow: Subscriber;

    // TODO: Provide links (maybe mapped)
    fn create() -> Self;

    fn on_sub(&mut self, _event: &SubEvent<Self::Flow>) {}

    fn render(&self, state: &Self::Flow) -> Option<Html>;
}
