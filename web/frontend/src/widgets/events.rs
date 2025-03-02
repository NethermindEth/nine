use derive_more::From;
use ui9_dui::{Sub, SubEvent, State};
use ui9_dui::tracers::event::Event;
use yew::{html, Component, Context, Html};

pub struct EventsWidget {
    sub: Sub<Event>,
    state: Option<State<Event>>,
    lost: bool,
}

#[derive(Debug, From)]
pub enum Msg {
    Event(SubEvent<Event>),
}

impl Component for EventsWidget {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // TODO: Use props here to get FQN
        let mut sub = Sub::<Event>::local_unified();
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        log::info!("Message: {msg:?}");
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

impl EventsWidget {
    fn render(&self) -> Option<Html> {
        let state = self.state.as_ref()?.borrow();
        Some(html! {
            <div>{ format!("events: {state:?}") }</div>
        })
    }
}
