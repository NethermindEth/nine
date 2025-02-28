use yew::{html, Component, Context, Html};

pub struct WebApp {}

impl WebApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for WebApp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p>{ "N9 APP" }</p>
        }
    }
}
