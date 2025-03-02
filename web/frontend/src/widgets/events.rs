use derive_more::From;
use ui9::names::Fqn;
use ui9_dui::{Sub, SubEvent, State, Subscriber, Unified};
use yew::{html, Component, Context, Html, Properties};

pub struct SubWidget<E: Subscriber> {
    sub: Sub<E>,
    state: Option<State<E>>,
    lost: bool,
}

#[derive(From)]
pub enum Msg<E: Subscriber> {
    Event(SubEvent<E>),
}

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub fqn: Fqn,
}

impl<E: Subscriber + Unified> Component for SubWidget<E> {
    type Message = Msg<E>;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // TODO: Use props here to get FQN
        let fqn = ctx.props().fqn.clone();
        let mut sub = Sub::<E>::local(fqn);
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

impl<E: Subscriber> SubWidget<E> {
    fn render(&self) -> Option<Html> {
        let state = self.state.as_ref()?.borrow();
        let typ = std::any::type_name::<E>();
        Some(html! {
            <div>{ format!("Loaded: {typ}") }</div>
        })
    }
}
