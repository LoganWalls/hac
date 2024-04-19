use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::Component;

pub struct TerminalTooSmall<'a> {
    colors: &'a colors::Colors,
}

pub struct TerminalTooSmallLayout {}

impl<'a> TerminalTooSmall<'a> {
    pub fn new(colors: &'a colors::Colors) -> Self {
        TerminalTooSmall { colors }
    }
}

impl Component for TerminalTooSmall<'_> {
    fn draw(&mut self, frame: &mut Frame, size: Rect) -> anyhow::Result<()> {
        let layout = build_layout(size);

        let lines = Line::from("Terminal is too small:".bold().fg(self.colors.bright.black));
        let curr_size = Line::from(vec![
            "Width = ".bold().fg(self.colors.bright.black),
            format!("{} ", size.width).bold().fg(self.colors.normal.red),
            "Height = ".bold().fg(self.colors.bright.black),
            format!("{}", size.height).bold().fg(self.colors.normal.red),
        ]);
        let empty = Line::from(" ");
        let hint = Line::from("Minimum size needed:".bold().fg(self.colors.bright.black));
        let min_size = Line::from("Width = 80 Height = 22".bold().fg(self.colors.bright.black));

        let text = Paragraph::new(vec![lines, curr_size, empty, hint, min_size])
            .wrap(Wrap { trim: true })
            .centered()
            .alignment(Alignment::Center);

        frame.render_widget(text, layout);

        Ok(())
    }
}

fn build_layout(size: Rect) -> Rect {
    Layout::default()
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Fill(1),
        ])
        .direction(Direction::Vertical)
        .flex(Flex::Center)
        .split(size)[1]
}
