use ratatui::{crossterm, DefaultTerminal, Frame};

use crate::cli::setup::setup_system;

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let (memory, cpu) = setup_system(8196);

    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

