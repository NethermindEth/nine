use super::component::SubComponent;
use derive_more::From;
use ui9::names::Fqn;
use ui9_dui::{State, Sub, SubEvent, Subscriber};
use yew::{html, Component, Context, Html, Properties};

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
            Msg::Event(event) => {
                // Events processing
                match event {
                    SubEvent::State(state) => {
                        self.state = Some(state);
                        self.lost = false;
                    }
                    SubEvent::Event(event) => {}
                    SubEvent::Lost => {
                        self.lost = true;
                    }
                }
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.state
            .as_ref()
            .and_then(|state| self.component.render(state))
            .unwrap_or_else(|| {
                html! {
                    <div>
                        <img src="static/loader.gif" />
                    </div>
                }
            })
    }
}
