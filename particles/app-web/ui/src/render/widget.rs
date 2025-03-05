use super::projection::Projection;
use std::any::type_name;
use yew::{html, Component, Context, Html};

pub trait SubComponent: 'static {
    type Projection: Projection;

    // TODO: Provide links (maybe mapped)
    fn create() -> Self;

    /*
    fn on_sub(&mut self, _event: &Self::Projection::Event) {}

    */

    fn render(&self, state: <Self::Projection as Projection>::State<'_>) -> Option<Html>;
}

pub struct SubWidget<C: SubComponent> {
    component: C,
    projection: C::Projection,
}

impl<C: SubComponent> Component for SubWidget<C> {
    type Message = <C::Projection as Projection>::Message;
    type Properties = <C::Projection as Projection>::Properties;

    fn create(ctx: &Context<Self>) -> Self {
        let component = C::create();
        let mut projection = C::Projection::create(ctx.props());
        for stream in projection.streams() {
            ctx.link().send_stream(stream);
        }
        Self {
            component,
            projection,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.projection.update(msg)
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.projection
            .state()
            .and_then(|state| self.component.render(state))
            .unwrap_or_else(|| {
                let name = type_name::<C>();
                html! {
                    <div class="spinner">
                        <img width="32px" src="static/loader.gif" />
                        <div>{ "Loading..." }</div>
                        <div>{ name }</div>
                    </div>
                }
            })
    }
}
