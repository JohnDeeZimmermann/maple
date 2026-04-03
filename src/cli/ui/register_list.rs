use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    cli::ui::state::{format_value, AppState, PaneKind},
    maple::cpu::MapleCPU,
};

pub fn render_register_list(frame: &mut Frame, area: Rect, state: &mut AppState, cpu: &MapleCPU) {
    let border_style = if state.active_pane == PaneKind::RegisterList {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Gray)
    };

    let block = Block::default()
        .title("Registers")
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let max_register: u32 = 16;
    let visible_rows = inner.height as u32;
    let max_scroll = max_register.saturating_sub(visible_rows);
    let scroll_offset = state.register_scroll_offset.min(max_scroll);
    state.register_scroll_visible_rows = visible_rows;

    let mut lines: Vec<Line> = Vec::new();

    for row in 0..visible_rows {
        let register = scroll_offset + row;
        if register >= max_register {
            break;
        }

        let name = REGISTER_NAMES[register as usize];
        let value = cpu.get_register(row as u8);
        let is_selected = row == state.selected_register;
        let value_str = format_value(value, &state.format_register_values);

        let mut style = Style::default();

        if is_selected {
            style = style.add_modifier(Modifier::REVERSED);
        }

        let mut spans = Vec::new();
        spans.push(Span::styled(format!("{} ", name), style));
        spans.push(Span::styled(value_str, style));

        let line = Line::from(spans);

        lines.push(line);
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

const REGISTER_NAMES: &[&str; 16] = &[
    "R0", "R1", "R2", "R3", "R4", "R5", "SP", "PC", "DL", "CR", "IOP", "PB", "SY", "FP", "H0", "H1",
];
