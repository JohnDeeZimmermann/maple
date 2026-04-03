use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::cli::ui::state::{format_address, format_value, AppState, PaneKind};
use crate::maple::memory::Memory;

struct RegisterMarker {
    label: &'static str,
    color: Color,
}

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
        let register_markers = get_register_markers(state, address as u64);

        let address_str = format_address(address as u64, &state.format_memory_addresses);
        let value_str = format_value(value, &state.format_memory);

        let mut style = Style::default();

        if is_pc {
            style = style.bg(Color::Green).fg(Color::Black);
        }

        if is_selected {
            style = style.add_modifier(Modifier::REVERSED);
        }

        let mut spans = render_register_markers(&register_markers, style);
        spans.push(Span::styled(format!("{} ", address_str), style));
        spans.push(Span::styled(value_str, style));

        let line = Line::from(spans);

        lines.push(line);
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

fn get_register_markers(state: &AppState, address: u64) -> Vec<RegisterMarker> {
    let mut markers = Vec::new();

    if state.stack_pointer == address {
        markers.push(RegisterMarker {
            label: "sp",
            color: Color::Yellow,
        });
    }

    if state.frame_pointer == address {
        markers.push(RegisterMarker {
            label: "fp",
            color: Color::Cyan,
        });
    }

    if state.dynamic_link == address {
        markers.push(RegisterMarker {
            label: "dl",
            color: Color::Magenta,
        });
    }

    if state.page_table_base == address {
        markers.push(RegisterMarker {
            label: "pb",
            color: Color::Blue,
        });
    }

    if state.interrupt_table_base == address {
        markers.push(RegisterMarker {
            label: "ib",
            color: Color::Red,
        });
    }

    markers
}

fn render_register_markers(markers: &[RegisterMarker], row_style: Style) -> Vec<Span<'static>> {
    if markers.is_empty() {
        return vec![Span::styled("      ", row_style)];
    }

    let mut spans = Vec::new();
    spans.push(Span::styled("[", row_style));

    for (index, marker) in markers.iter().enumerate() {
        if index > 0 {
            spans.push(Span::styled(" ", row_style));
        }

        spans.push(Span::styled(marker.label, row_style.fg(marker.color)));
    }

    spans.push(Span::styled("]  ", row_style));

    spans
}
