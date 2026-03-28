use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame,
};

use crate::cli::ui::app::{AppState, PaneKind};

pub fn render_memory_list(frame: &mut Frame, area: Rect, state: &AppState) {
    let border_style = if state.active_pane == PaneKind::MemoryList {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Gray)
    };

    let block = Block::default()
        .title("Memory")
        .borders(Borders::ALL)
        .border_style(border_style);

    frame.render_widget(block, area);
}
