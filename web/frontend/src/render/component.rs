use ui9_dui::Subscriber;
use yew::Html;

pub trait SubComponent: 'static {
    type Flow: Subscriber;

    fn render(&self) -> Option<Html>;
}
