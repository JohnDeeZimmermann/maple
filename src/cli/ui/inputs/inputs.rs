use ratatui::crossterm::event::KeyEvent;

use crate::cli::ui::{
    inputs::{
        global_inputs::handle_global_inputs, memory_pane_inputs::handle_memory_pane_input,
        register_pane_inputs::handle_register_pane_input,
    },
    state::{AppState, PaneKind},
};

pub fn handle_input(key: KeyEvent, state: &mut AppState) -> bool {
    match state.active_pane {
        PaneKind::RegisterList => {
            if handle_register_pane_input(key, state) {
                return true;
            }
        }
        PaneKind::MemoryList => {
            if handle_memory_pane_input(key, state) {
                return true;
            }
        }
        PaneKind::CommandLine => {}
    }

    if handle_global_inputs(key, state) {
        return true;
    }

    false
}
