use crate::widgets::{Component, Reason};
use crb::core::time::Duration;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Widget},
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

        let items: Vec<ListItem> = state
            .events
            .iter()
            .rev()
            .map(|failure| {
                ListItem::new(vec![Line::from(vec![Span::styled(
                    &failure.reason,
                    Style::default().fg(Color::Red),
                )])])
            })
            .collect();

        let list = List::new(items);
        list.render(area, buf);

        Ok(())
    }
}
