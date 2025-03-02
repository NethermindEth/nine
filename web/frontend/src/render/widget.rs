use super::component::SubComponent;
use derive_more::From;
use ui9::names::Fqn;
use ui9_dui::{Sub, SubEvent, State, Subscriber};
use yew::{html, Component, Context, Html, Properties};

pub struct SubWidget<C: SubComponent> {
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
        // TODO: Use props here to get FQN
        let fqn = ctx.props().fqn.clone();
        let mut sub = Sub::<C::Flow>::local(fqn);
        if let Ok(stream) = sub.events() {
            log::info!("Subscribed to events");
            ctx.link().send_stream(stream);
        }
        Self {
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
                    SubEvent::Event(event) => {
                    }
                    SubEvent::Lost => {
                        self.lost = true;
                    }
                }
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.render()
            .unwrap_or_else(|| html! {
                <div>{ "Loading..." }</div>
            })
    }
}

impl<C: SubComponent> SubWidget<C> {
    fn render(&self) -> Option<Html> {
        let state = self.state.as_ref()?.borrow();
        let typ = std::any::type_name::<C::Flow>();
        Some(html! {
            <div>{ format!("Loaded: {typ}") }</div>
        })
    }
}
