use crb::core::watch::Ref;
use derive_more::{Deref, From};
use futures::{Stream, StreamExt};
use std::any::type_name;
use std::pin::Pin;
use ui9::names::Fqn;
use ui9_dui::{State, Sub, SubEvent, Subscriber};
use yew::{html, Component, Context, Html, Properties};

pub trait SubComponent: 'static {
    type Projection: Projection;

    // TODO: Provide links (maybe mapped)
    fn create() -> Self;

    /*
    fn on_sub(&mut self, _event: &Self::Projection::Event) {}

    */

    fn render(&self, state: <Self::Projection as Projection>::State<'_>) -> Option<Html>;
}

pub trait Projection {
    type Message;
    type Properties: Properties;
    type State<'a>
    where
        Self: 'a;

    fn create(props: &Self::Properties) -> Self;

    fn streams(&mut self) -> Vec<Pin<Box<dyn Stream<Item = Self::Message>>>>;

    fn update(&mut self, msg: Self::Message) -> bool;

    fn state(&self) -> Option<Self::State<'_>>;
}

impl Projection for () {
    type Message = ();
    type Properties = ();
    type State<'a> = ();

    fn create(props: &Self::Properties) -> Self {
        ()
    }

    fn streams(&mut self) -> Vec<Pin<Box<dyn Stream<Item = Self::Message>>>> {
        Vec::new()
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        true
    }

    fn state(&self) -> Option<Self::State<'_>> {
        Some(())
    }
}

pub struct SingleFlow<F: Subscriber> {
    sub: Sub<F>,
    state: Option<State<F>>,
    lost: bool,
}

impl<F: Subscriber> Projection for SingleFlow<F> {
    type Message = Msg<F>;
    type Properties = Props;
    type State<'a> = SingleState<'a, F>;

    fn create(props: &Self::Properties) -> Self {
        let fqn = props.fqn.clone();
        let sub = Sub::<F>::local(fqn);
        Self {
            sub,
            state: None,
            lost: false,
        }
    }

    fn streams(&mut self) -> Vec<Pin<Box<dyn Stream<Item = Self::Message>>>> {
        let stream = self.sub.events().unwrap().map(Msg::Event).boxed();
        vec![stream]
    }

    // TODO: Provide a reference to a component
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Event(sub_event) => {
                // self.component.on_sub(&sub_event);
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

    fn state(&self) -> Option<Self::State<'_>> {
        let state = self.state.as_ref()?.borrow();
        Some(SingleState {
            state,
            lost: self.lost,
        })
    }
}

#[derive(From)]
pub enum Msg<F: Subscriber> {
    Event(SubEvent<F>),
}

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub fqn: Fqn,
}

#[derive(Deref)]
pub struct SingleState<'a, F> {
    #[deref]
    state: Ref<'a, F>,
    lost: bool,
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
