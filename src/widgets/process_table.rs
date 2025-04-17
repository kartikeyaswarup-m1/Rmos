// src/widgets/process_table.rs
use ratatui::{
    widgets::{Block, Borders, Table, Row, Cell},
    layout::Rect,
    Frame,
    style::{Style, Modifier},
};
use sysinfo::{ProcessExt, System};

pub fn draw_process_table<B: ratatui::backend::Backend>(f: &mut Frame<B>, area: Rect, sys: &System) {
    let rows: Vec<Row> = sys.processes()
        .map(|(pid, process)| {
            Row::new(vec![
                Cell::from(pid.to_string()),
                Cell::from(process.name().to_string()),
                Cell::from(format!("{:.2}%", process.cpu_usage())),
                Cell::from(format!("{:.2} MB", process.memory() as f64 / 1024.0)),
            ])
        })
        .collect();

    let table = Table::new(rows)
        .header(Row::new(vec!["PID", "Name", "CPU", "Memory"]).style(Style::default().add_modifier(Modifier::BOLD)))
        .block(Block::default().title("Processes").borders(Borders::ALL));

    f.render_widget(table, area);
}
