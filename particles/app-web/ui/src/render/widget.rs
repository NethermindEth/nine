use derive_more::From;
use std::any::type_name;
use ui9::names::Fqn;
use ui9_dui::{State, Sub, SubEvent, Subscriber};
use yew::{html, Component, Context, Html, Properties};

pub trait SubComponent: 'static {
    type Flow: Subscriber;

    // TODO: Provide links (maybe mapped)
    fn create() -> Self;

    fn on_sub(&mut self, _event: &SubEvent<Self::Flow>) {}

    fn render(&self, state: &Self::Flow) -> Option<Html>;
}

pub struct SubWidget<C: SubComponent> {
    component: C,
    sub: Sub<C::Flow>,
    state: Option<State<C::Flow>>,
    lost: bool,
}

#[derive(From)]
pub enum Msg<C: SubComponent> {
    Event(SubEvent<C::Flow>),
}

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub fqn: Fqn,
}

impl<C: SubComponent> Component for SubWidget<C> {
    type Message = Msg<C>;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let component = C::create();
        let fqn = ctx.props().fqn.clone();
        let mut sub = Sub::<C::Flow>::local(fqn);
        if let Ok(stream) = sub.events() {
            log::info!("Subscribed to events");
            ctx.link().send_stream(stream);
        }
        Self {
            component,
            sub,
            state: None,
            lost: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Event(sub_event) => {
                self.component.on_sub(&sub_event);
                // Events processing
                match sub_event {
                    SubEvent::State(state) => {
                        self.state = Some(state);
                        self.lost = false;
                    }
                    SubEvent::Event(_event) => {}
                    SubEvent::Lost => {
                        self.lost = true;
                    }
                }
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let name = type_name::<C::Flow>();
        self.state
            .as_ref()
            .and_then(|state| {
                let state = state.borrow();
                self.component.render(&state)
            })
            .unwrap_or_else(|| {
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
