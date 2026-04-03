use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::cli::ui::state::{AppState, PaneKind};

pub fn handle_global_inputs(key: KeyEvent, state: &mut AppState) -> bool {
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

    if key.code == KeyCode::Enter {
        state.steps_to_execute += 1;
        return false;
    }

    false
}
