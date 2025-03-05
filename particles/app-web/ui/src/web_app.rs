use crate::widgets::dashboard::{Dashboard, DashboardWidget};
use ui9_dui::Unified;
use ui9_net::tracers::peer::{Peer, PeerId};
use yew::{html, Component, Context, Html};

pub struct WebApp {}

impl Component for WebApp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <DashboardWidget fqn={Dashboard::fqn()} />
        }
    }
}
