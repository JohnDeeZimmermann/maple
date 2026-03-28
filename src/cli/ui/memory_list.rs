use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::cli::ui::app::{AppState, BinaryFormat, PaneKind};
use crate::maple::memory::Memory;

pub fn render_memory_list(frame: &mut Frame, area: Rect, state: &mut AppState, memory: &Memory) {
    let border_style = if state.active_pane == PaneKind::MemoryList {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Gray)
    };

    let block = Block::default()
        .title("Memory")
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let visible_rows = inner.height as u32;
    let max_scroll = state.max_address.saturating_sub(visible_rows);
    let scroll_offset = state.memory_scroll_offset.min(max_scroll);

    state.memory_scroll_visible_rows = visible_rows;

    let mut lines: Vec<Line> = Vec::new();

    for row in 0..visible_rows {
        let address = scroll_offset + row;
        if address >= state.max_address {
            break;
        }

        let value = memory.get(address);
        let is_pc = address as u64 == state.program_counter;
        let is_selected = address == state.selected_address;

        let address_str = format_address(address as u64, &state.format_memory_addresses);
        let value_str = format_value(value, &state.format_memory);

        let mut style = Style::default();

        if is_pc {
            style = style.bg(Color::Green).fg(Color::Black);
        }

        if is_selected {
            style = style.add_modifier(Modifier::REVERSED);
        }

        let line = Line::from(vec![
            Span::styled(format!("{} ", address_str), style),
            Span::styled(value_str, style),
        ]);

        lines.push(line);
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn format_address(value: u64, format: &BinaryFormat) -> String {
    match format {
        BinaryFormat::Hex => format!("0x{:08X}", value),
        BinaryFormat::Decimal => format!("{:10}", value),
        BinaryFormat::Binary => format!("0b{:032b}", value),
    }
}

fn format_value(value: u64, format: &BinaryFormat) -> String {
    match format {
        BinaryFormat::Hex => format!("0x{:016X}", value),
        BinaryFormat::Decimal => format!("{}", value),
        BinaryFormat::Binary => format!("0b{:064b}", value),
    }
}
