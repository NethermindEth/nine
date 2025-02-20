use super::liner::Liner;
use crate::widgets::{Component, Reason};
use crb::core::time::Duration;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph, Widget, Wrap},
};
use ui9_app::SubState;
use ui9_dui::tracers::failure::Failure;

pub struct FailureLog {
    state: SubState<Failure>,
}

impl FailureLog {
    pub fn new() -> Self {
        Self {
            state: SubState::new_local_unified(),
        }
    }
}

impl Component for FailureLog {
    fn title(&self) -> Option<&str> {
        Some("Failure")
    }

    fn render(&self, area: Rect, buf: &mut Buffer) -> Result<(), Reason> {
        let ported = self.state.borrow();
        let state = ported.state()?;

        let mut liner = Liner::new();

        let mut log = String::new();
        let items = state.events.iter().rev().for_each(|failure| {
            liner.add_line(&failure.reason);
        });

        let paragraph = Paragraph::new(liner.text()).wrap(Wrap { trim: true });

        paragraph.render(area, buf);

        Ok(())
    }
}
