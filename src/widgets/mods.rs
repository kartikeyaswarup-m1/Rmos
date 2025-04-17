use ratatui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
    Frame,
};

use crate::system::SystemData;

pub fn draw_widgets(f: &mut Frame<'_>, sys: &SystemData, cpu_area: Rect, mem_area: Rect) {
    // CPU Usage
    let cpu = sys.cpu_usage();
    let cpu_gauge = Gauge::default()
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(cpu as u16);
    f.render_widget(cpu_gauge, cpu_area);

    // Memory Usage
    let (used, total) = sys.memory();
    let percent = if total > 0 {
        ((used as f64 / total as f64) * 100.0) as u16
    } else {
        0
    };

    let mem_gauge = Gauge::default()
        .block(Block::default().title("Memory Usage").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Blue))
        .percent(percent);
    f.render_widget(mem_gauge, mem_area);
}
