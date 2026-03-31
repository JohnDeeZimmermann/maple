use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::cli::ui::app::{AppState, PaneKind};

const PADDING: u32 = 10;

pub fn handle_memory_pane_input(key: KeyEvent, state: &mut AppState) -> bool {
    // Memory pane specific inputs

    if state.active_pane != PaneKind::MemoryList {
        return false;
    }

    match key.code {
        KeyCode::Up => handle_up(state),
        KeyCode::Down => handle_down(state),
        KeyCode::Tab => handle_tab(key, state),
        _ => {}
    }

    false
}

fn handle_tab(key: KeyEvent, state: &mut AppState) {
    if key.modifiers.contains(KeyModifiers::SHIFT) {
        state.format_memory_addresses = state.format_memory_addresses.cycle();
    } else {
        state.format_memory = state.format_memory.cycle();
    }
}

fn handle_up(state: &mut AppState) {
    if state.selected_address > 0 {
        state.selected_address -= 1;

        if state.selected_address < state.memory_scroll_offset + PADDING
            && state.memory_scroll_offset > 0
        {
            state.memory_scroll_offset = state.memory_scroll_offset.saturating_sub(1);
        }
    }
}

fn handle_down(state: &mut AppState) {
    if state.selected_address < state.max_address - 1 {
        state.selected_address += 1;

        // If selection is within PADDING rows from bottom of visible area, scroll down
        let bottom_visible = state.memory_scroll_offset + state.memory_scroll_visible_rows;
        if state.selected_address >= bottom_visible - PADDING {
            let max_scroll = state
                .max_address
                .saturating_sub(state.memory_scroll_visible_rows);
            state.memory_scroll_offset = (state.memory_scroll_offset + 1).min(max_scroll);
        }
    }
}
