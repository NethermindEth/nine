use ui9_dui::Sub;
use ui9_dui::tracers::event::Event;
use yew::{html, Component, Context, Html};

pub struct EventsWidget {
    sub: Sub<Event>,
}

impl Component for EventsWidget {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // TODO: Use props here to get FQN
        let sub = Sub::<Event>::local_unified();
        Self {
            sub,
        }
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
        /*
        let ported = self.state.borrow();
        let state = ported.state().ok()?;
        */
        Some(html! {
            <div>{ "events" }</div>
        })
    }
}
