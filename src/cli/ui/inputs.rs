use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::cli::ui::app::{AppState, PaneKind};

const PADDING: u32 = 5;

pub fn handle_input(key: KeyEvent, state: &mut AppState) -> bool {
    if handle_global(key, state) {
        return true;
    }

    if handle_memory_pane(key, state) {
        return true;
    }

    if key.code == KeyCode::Enter {
        state.steps_to_execute += 1;
        return false;
    }

    false
}

fn handle_global(key: KeyEvent, state: &mut AppState) -> bool {
    // Global shortcuts
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('m') => {
                state.active_pane = PaneKind::MemoryList;
                return false;
            }
            KeyCode::Char('r') => {
                state.active_pane = PaneKind::RegisterList;
                return false;
            }
            KeyCode::Char('q') => return true,
            _ => {}
        }
    }

    false
}

fn handle_memory_pane(key: KeyEvent, state: &mut AppState) -> bool {
    // Memory pane specific inputs
    if state.active_pane == PaneKind::MemoryList {
        match key.code {
            KeyCode::Up => handle_up(state),
            KeyCode::Down => handle_down(state),
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    state.format_memory_addresses = state.format_memory_addresses.cycle();
                } else {
                    state.format_memory = state.format_memory.cycle();
                }
            }
            _ => {}
        }
    }

    false
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
