use crate::render::{single, SubComponent, SubContext, SubWidget};
use crb::core::uuid::Uuid;
use n9_control_session::SessionControl;
use ui9::names::Fqn;
use ui9_dui::tracers::event::Event;
use yew::{html, Html};

pub type SessionControlWidget = SubWidget<SessionControlComponent>;

pub struct SessionControlComponent {}

#[derive(Clone)]
pub enum Msg {
    NewChat,
}

impl SubComponent for SessionControlComponent {
    type Projection = single::Flow<SessionControl>;
    type Message = Msg;

    fn create() -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message, pro: &mut Self::Projection) -> bool {
        match msg {
            Msg::NewChat => {
                let fqn: Fqn = vec!["user-chat".to_string(), Uuid::new_v4().to_string()].into();
                pro.new_chat(fqn);
                false
            }
        }
    }

    fn render(&self, state: single::State<SessionControl>, ctx: &SubContext<Self>) -> Option<Html> {
        let typ = std::any::type_name::<Event>();
        let onclick = ctx.event(Msg::NewChat);
        Some(html! {
            <div class="widget-session-control">
                <div class="widget-session-control-header">
                    <div class="widget-session-control-header-title">
                        { "Chats" }
                    </div>
                    <div class="widget-session-control-header-new" {onclick}>
                        { "New" }
                    </div>
                </div>
            </div>
        })
    }
}
