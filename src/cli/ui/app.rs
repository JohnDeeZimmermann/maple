use ratatui::{
    crossterm,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    DefaultTerminal, Frame,
};

use crate::cli::{setup::setup_system, ui::memory_list::render_memory_list};

#[derive(Debug, PartialEq)]
pub enum PaneKind {
    MemoryList,
    RegisterList,
    CommandLine,
}

#[derive(Debug, PartialEq)]
pub enum BinaryFormat {
    Hex,
    Binary,
    Decimal,
}

pub struct AppState {
    pub active_pane: PaneKind,
    pub selected_address: u32,
    pub selected_register: u32,
    pub max_address: u32,
    pub format_memory_addresses: BinaryFormat,
    pub format_memory: BinaryFormat,
    pub format_registers: BinaryFormat,
}

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let max_address = 8196;

    let (memory, cpu) = setup_system(max_address);
    let state = AppState {
        selected_address: 0,
        selected_register: 0,
        max_address,
        active_pane: PaneKind::MemoryList,
        format_memory_addresses: BinaryFormat::Hex,
        format_memory: BinaryFormat::Decimal,
        format_registers: BinaryFormat::Decimal,
    };

    loop {
        terminal.draw(|frame| render(frame, &state))?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, state: &AppState) {
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
    render_content(frame, content, state);
    render_footer(frame, footer);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let header = Block::default().title("Header").borders(Borders::ALL);
    frame.render_widget(header, area);
}

fn render_content(frame: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default();

    let inner = block.inner(area);

    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);

    let memory_list = chunks[0];
    let register_list = chunks[1];

    render_memory_list(frame, memory_list, state);
    render_register_list(frame, register_list);
}

fn render_register_list(frame: &mut Frame, area: Rect) {}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Block::default().title("Actions").borders(Borders::ALL);
    frame.render_widget(footer, area);
}
