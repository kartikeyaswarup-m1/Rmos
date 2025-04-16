use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::system::SystemData;

pub fn render<B: Backend>(f: &mut Frame<'_>, sys: &SystemData) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(size);

    let cpu = sys.cpu_usage();
    let mem_used = sys.used_memory();
    let mem_total = sys.total_memory();

    let cpu_text = vec![
        Line::from(vec![Span::styled("CPU Usage", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from(Span::raw(format!("{:.2}%", cpu))),
    ];

    let mem_text = vec![
        Line::from(vec![Span::styled("Memory Usage", Style::default().add_modifier(Modifier::BOLD))]),
        Line::from(Span::raw(format!("{}/{} KB", mem_used, mem_total))),
    ];

    let cpu_paragraph = Paragraph::new(cpu_text).block(Block::default().title("CPU").borders(Borders::ALL));
    let mem_paragraph = Paragraph::new(mem_text).block(Block::default().title("Memory").borders(Borders::ALL));

    f.render_widget(cpu_paragraph, chunks[0]);
    f.render_widget(mem_paragraph, chunks[1]);
}
