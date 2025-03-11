use crate::render::{single, SubComponent, SubContext, SubWidget};
use n9_control_session::SessionControl;
use ui9_dui::tracers::event::Event;
use yew::{html, Html};

pub type SessionControlWidget = SubWidget<SessionControlComponent>;

pub struct SessionControlComponent {}

impl SubComponent for SessionControlComponent {
    type Projection = single::Flow<SessionControl>;
    type Message = ();

    fn create() -> Self {
        Self {}
    }

    fn render(
        &self,
        state: single::State<SessionControl>,
        _ctx: &SubContext<Self>,
    ) -> Option<Html> {
        let typ = std::any::type_name::<Event>();
        Some(html! {
            <div class="widget-session-control">
                <div class="widget-session-control-header">
                    <div class="widget-session-control-header-title">
                        { "Chats" }
                    </div>
                    <div class="widget-session-control-header-new">
                        { "New" }
                    </div>
                </div>
            </div>
        })
    }
}
