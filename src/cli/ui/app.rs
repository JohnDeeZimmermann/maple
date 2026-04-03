use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

use crate::cli::{
    setup::setup_system,
    ui::{
        inputs::inputs::handle_input, memory_list::render_memory_list,
        register_list::render_register_list, state::AppState,
    },
};
use crate::maple::cpu::MapleCPU;
use crate::maple::memory::Memory;

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let max_address = 8196;

    let (mut cpu, mut memory) = setup_system(max_address);

    let mut state = AppState::new(max_address);

    loop {
        while state.steps_to_execute > 0 {
            let result = cpu.process(&mut memory);
            state.steps_to_execute -= 1;
            if result == crate::maple::cpu::ExecutionResult::Exit {
                break;
            }
        }

        state.sync_from_cpu(&cpu);
        terminal.draw(|frame| render(frame, &mut state, &memory, &cpu))?;

        if let Event::Key(key) = event::read()? {
            if handle_input(key, &mut state) {
                break Ok(());
            }
        }
    }
}

fn render(frame: &mut Frame, state: &mut AppState, memory: &Memory, cpu: &MapleCPU) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(100),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = chunks[0];
    let content = chunks[1];
    let footer = chunks[2];

    render_header(frame, header);
    render_content(frame, content, state, memory, cpu);
    render_footer(frame, footer, state);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let header = Block::default().title("Header").borders(Borders::ALL);
    frame.render_widget(header, area);
}

fn render_content(
    frame: &mut Frame,
    area: Rect,
    state: &mut AppState,
    memory: &Memory,
    cpu: &MapleCPU,
) {
    let block = Block::default();

    let inner = block.inner(area);

    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);

    let memory_list = chunks[0];
    let register_list = chunks[1];

    render_memory_list(frame, memory_list, state, memory);
    render_register_list(frame, register_list, state, cpu);
}

fn render_footer(frame: &mut Frame, area: Rect, _state: &AppState) {
    let status_text = "Enter: Step | C-M: Memory | C-R: Registers | C-Q: Quit";

    let footer = Block::default().title("Actions").borders(Borders::ALL);

    let inner = footer.inner(area);
    frame.render_widget(footer, area);

    let line = Line::from(vec![Span::styled(
        status_text,
        Style::default().fg(Color::White),
    )]);
    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, inner);
}
