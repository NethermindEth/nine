use ui9_dui::{Subscriber, State};
use yew::Html;

pub trait SubComponent: 'static {
    type Flow: Subscriber;

    // TODO: Provide links (maybe mapped)
    fn create() -> Self;

    fn render(&self, state: &State<Self::Flow>) -> Option<Html>;
}
