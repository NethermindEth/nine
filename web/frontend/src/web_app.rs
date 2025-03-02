use crate::widgets::{EventsList, PeersList};
use yew::{html, Component, Context, Html};
use ui9_dui::tracers::event::Event;
use ui9_dui::Unified;

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
        let fqn = Event::fqn();
        html! {
            <div>
                <EventsList fqn={fqn.clone()} />
                <PeersList {fqn} />
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
