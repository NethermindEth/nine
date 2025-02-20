use ratatui::text::{Line, Span, Text};

pub struct Liner<'a> {
    all_lines: Vec<Line<'a>>,
    current_line: Vec<Span<'a>>,
}

impl<'a> Liner<'a> {
    pub fn new() -> Self {
        Self {
            all_lines: Vec::new(),
            current_line: Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: &'a str) {
        self.all_lines.push(Line::from(line));
    }

    pub fn text(mut self) -> Text<'a> {
        Text::from(self.all_lines)
    }
}
