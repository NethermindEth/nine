use crate::widgets::PeersList;
use ui9_dui::Unified;
use ui9_net::tracers::peer::Peer;
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
            <div class="app">
                <div class="app-header">
                    <div class="loader">
                        <div class="loader-container">
                            <img src="static/logo.png" />
                            <div class="loader-overlay"></div>
                        </div>
                    </div>
                    <div class="app-header-title">{ "N9 Dashboard" }</div>
                </div>

                <div class="app-content">
                    // <EventsList fqn={Event::fqn()} />
                    <PeersList fqn={Peer::fqn()} />
                </div>
            </div>
        }
    }
}
