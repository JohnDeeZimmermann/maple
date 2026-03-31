use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::cli::ui::app::{AppState, PaneKind};

const PADDING: u32 = 5;

pub fn handle_register_pane_input(key: KeyEvent, state: &mut AppState) -> bool {
    if state.active_pane != PaneKind::RegisterList {
        return false;
    }

    match key.code {
        KeyCode::Up => handle_up(state),
        KeyCode::Down => handle_down(state),
        KeyCode::Tab => handle_tab(state),
        _ => {}
    }

    return false;
}

fn handle_tab(state: &mut AppState) {
    state.format_register_values = state.format_register_values.cycle();
}

fn handle_up(state: &mut AppState) {
    if state.selected_register > 0 {
        state.selected_register -= 1;

        if state.selected_register < state.register_scroll_offset + PADDING
            && state.register_scroll_offset > 0
        {
            state.register_scroll_offset = state.register_scroll_offset.saturating_sub(1);
        }
    }
}

fn handle_down(state: &mut AppState) {
    if state.selected_register < 16 - 1 {
        state.selected_register += 1;

        // If selection is within PADDING rows from bottom of visible area, scroll down
        let bottom_visible = state.register_scroll_offset + state.register_scroll_visible_rows;
        if state.selected_register >= bottom_visible - PADDING {
            let max_scroll = 16_u32.saturating_sub(state.register_scroll_visible_rows);
            state.register_scroll_offset = (state.register_scroll_offset + 1).min(max_scroll);
        }
    }
}
