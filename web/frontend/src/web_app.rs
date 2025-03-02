use crate::widgets::events::EventsWidget;
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
            <div>
                <EventsWidget />
                <div class="loader">
                    <div class="loader-container">
                        <img src="static/logo.png" />
                        <div class="loader-overlay"></div>
                    </div>
                </div>
            </div>
        }
    }
}
