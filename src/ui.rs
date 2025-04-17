use ratatui::{layout::{Constraint, Direction, Layout}, Frame};
use crate::system::SystemData;
use crate::widgets::draw_widgets;

pub fn render(f: &mut Frame<'_>, sys: &SystemData) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.size());

    draw_widgets(f, sys, chunks[0], chunks[1]);
}
