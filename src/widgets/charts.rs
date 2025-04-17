// src/widgets/chart.rs
use ratatui::{
    widgets::{Block, Borders, Chart, Dataset, Axis, GraphType},
    layout::Rect,
    style::{Color, Style},
    symbols,
    Frame,
};

pub fn draw_chart<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect, data: &[(f64, f64)]) {
    let dataset = Dataset::default()
        .name("CPU Usage")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(data);

    let chart = Chart::new(vec![dataset])
        .block(Block::default().title("CPU Usage").borders(Borders::ALL))
        .x_axis(Axis::default().title("Time").bounds([0.0, 100.0]))
        .y_axis(Axis::default().title("Usage (%)").bounds([0.0, 100.0]));

    f.render_widget(chart, area);
}
