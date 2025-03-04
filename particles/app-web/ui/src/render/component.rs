use ui9_dui::{State, Subscriber};
use yew::Html;

pub trait SubComponent: 'static {
    type Flow: Subscriber;

    // TODO: Provide links (maybe mapped)
    fn create() -> Self;

    fn render(&self, state: &Self::Flow) -> Option<Html>;
}
