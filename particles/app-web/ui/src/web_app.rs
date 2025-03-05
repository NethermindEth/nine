use crate::render::FqnLink;
use crate::widgets::dashboard::{Dashboard, DashboardWidget};
use ui9_dui::Unified;
use yew::{html, Component, Context, Html};

pub struct WebApp {}

impl Component for WebApp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let link: FqnLink = Dashboard::fqn().into();
        html! {
            <DashboardWidget {link} />
        }
    }
}
