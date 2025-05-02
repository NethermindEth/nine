use crate::render::{double, SubComponent, SubContext, SubWidget};
use crate::widgets::dashboard::Dashboard;
use crb::core::uuid::Uuid;
use n9_control_session::{SessionControl, SessionInfo, SessionKey};
use ui9::names::Fqn;
use ui9_dui::tracers::event::Event;
use ui9_dui::FqnLink;
use yew::{html, Html};

pub type SessionControlWidget = SubWidget<SessionControlComponent>;

pub struct SessionControlComponent {}

#[derive(Clone)]
pub enum Msg {
    NewChat,
    Select(Fqn),
}

impl SubComponent for SessionControlComponent {
    type Projection = double::Flow<SessionControl, Dashboard>;
    type Message = Msg;

    fn create() -> Self {
        Self {}
    }

    fn update(
        &mut self,
        msg: Self::Message,
        pro: &mut Self::Projection,
        ctx: &SubContext<Self>,
    ) -> bool {
        match msg {
            Msg::NewChat => {
                let fqn: Fqn = vec!["user-chat".to_string(), Uuid::new_v4().to_string()].into();
                pro.first.new_chat(fqn.clone());
                ctx.send(Msg::Select(fqn));
                // TODO: Select(fqn)
                false
            }
            Msg::Select(fqn) => {
                let peer = pro.second.state_view().and_then(|view| view.active_peer);
                if let Some(peer) = peer {
                    let link = FqnLink::remote(fqn, peer);
                    pro.second.set_chat(Some(link));
                }
                false
            }
        }
    }

    fn render(
        &self,
        state: double::State<SessionControl, Dashboard>,
        ctx: &SubContext<Self>,
    ) -> Option<Html> {
        let onclick = ctx.event(Msg::NewChat);
        let mut items: Vec<_> = state.active_sessions.iter().collect();
        items.sort_by_key(|(_, info)| info.created);
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
                <div class="widget-session-control-list">
                    { for items.into_iter().rev().map(|(k, v)| self.render_item(k, v, ctx)) }
                </div>
            </div>
        })
    }
}

impl SessionControlComponent {
    fn render_item(&self, key: &SessionKey, info: &SessionInfo, ctx: &SubContext<Self>) -> Html {
        let onclick = ctx.event(Msg::Select(key.clone()));
        html! {
            <div {onclick} class="widget-session-control-list-item">
                { info.created.to_string() }
            </div>
        }
    }
}
