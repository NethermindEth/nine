use super::projection::Projection;
use derive_more::Deref;
use futures::StreamExt;
use std::any::type_name;
use yew::{html, Callback, Component, Context, Html};

#[derive(Deref)]
pub struct SubContext<'a, C: SubComponent> {
    context: &'a Context<SubWidget<C>>,
}

impl<'a, C: SubComponent> SubContext<'a, C> {
    pub fn event<IN>(&self, event: C::Message) -> Callback<IN>
    where
        C::Message: Clone,
    {
        self.link().callback(move |_| Msg::Component(event.clone()))
    }
}

pub trait SubComponent: Sized + 'static {
    type Projection: Projection;
    type Message;

    // TODO: Provide links (maybe mapped)
    fn create() -> Self;

    fn update(&mut self, _msg: Self::Message, pro: &mut Self::Projection) -> bool {
        true
    }

    /*
    fn on_sub(&mut self, _event: &Self::Projection::Event) {}

    */

    fn render(
        &self,
        state: <Self::Projection as Projection>::State<'_>,
        ctx: &SubContext<Self>,
    ) -> Option<Html>;
}

pub enum Msg<C: SubComponent> {
    Install,
    Projection(<C::Projection as Projection>::Message),
    Component(C::Message),
}

pub struct SubWidget<C: SubComponent> {
    component: C,
    // TODO: Wrap with an Option
    projection: Option<C::Projection>,
}

impl<C: SubComponent> Component for SubWidget<C> {
    type Message = Msg<C>;
    type Properties = <C::Projection as Projection>::Properties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::Install);
        let component = C::create();
        Self {
            component,
            projection: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old: &Self::Properties) -> bool {
        let update = ctx.props() != old;
        if update {
            self.projection.take();
            // TODO: Cancel streams
            ctx.link().send_message(Msg::Install);
        }
        update
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Install => {
                self.install(ctx);
                false
            }
            _ => {
                if let Some(projection) = self.projection.as_mut() {
                    match msg {
                        Msg::Projection(event) => projection.update(event),
                        Msg::Component(event) => self.component.update(event, projection),
                        _ => false,
                    }
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.view_opt(ctx).unwrap_or_else(|| {
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

impl<C: SubComponent> SubWidget<C> {
    fn install(&mut self, ctx: &Context<Self>) {
        self.projection.take();
        let mut projection = C::Projection::create(ctx.props());
        for stream in projection.streams() {
            ctx.link().send_stream(stream.map(Msg::Projection));
        }
        self.projection = Some(projection);
    }

    fn view_opt(&self, ctx: &Context<Self>) -> Option<Html> {
        let ctx = SubContext { context: ctx };
        let projection = self.projection.as_ref()?;
        let state = projection.state()?;
        let rendered = self.component.render(state, &ctx)?;
        Some(rendered)
    }
}
