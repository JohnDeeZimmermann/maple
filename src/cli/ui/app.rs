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
    ui::{inputs::inputs::handle_input, memory_list::render_memory_list, register_list::render_register_list},
};
use crate::maple::cpu::MapleCPU;
use crate::maple::memory::Memory;

#[derive(Debug, PartialEq)]
pub enum PaneKind {
    MemoryList,
    RegisterList,
    CommandLine,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryFormat {
    Hex,
    Binary,
    Decimal,
}

impl BinaryFormat {
    pub fn cycle(&self) -> Self {
        match self {
            BinaryFormat::Hex => BinaryFormat::Decimal,
            BinaryFormat::Decimal => BinaryFormat::Binary,
            BinaryFormat::Binary => BinaryFormat::Hex,
        }
    }

    pub fn format(&self, value: u64) -> String {
        match self {
            BinaryFormat::Hex => format!("0x{:08X}", value),
            BinaryFormat::Decimal => format!("{}", value),
            BinaryFormat::Binary => format!("0b{:032b}", value),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            BinaryFormat::Hex => "Hex",
            BinaryFormat::Decimal => "Decimal",
            BinaryFormat::Binary => "Binary",
        }
    }
}

pub struct AppState {
    pub active_pane: PaneKind,
    pub selected_address: u32,
    pub selected_register: u32,
    pub memory_scroll_offset: u32,
    pub memory_scroll_visible_rows: u32,
    pub register_scroll_offset: u32,
    pub register_scroll_visible_rows: u32,
    pub program_counter: u64,
    pub max_address: u32,
    pub format_memory_addresses: BinaryFormat,
    pub format_memory: BinaryFormat,
    pub format_register_values: BinaryFormat,
    pub stack_pointer: u64,
    pub frame_pointer: u64,
    pub dynamic_link: u64,
    pub page_table_base: u64,
    pub interrupt_table_base: u64,
    pub steps_to_execute: u32,
}

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let max_address = 8196;

    let (mut cpu, mut memory) = setup_system(max_address);

    let mut state = AppState {
        selected_address: 0,
        selected_register: 0,
        memory_scroll_offset: 0,
        memory_scroll_visible_rows: 0,
        register_scroll_offset: 0,
        register_scroll_visible_rows: 0,
        program_counter: 0,
        max_address,
        active_pane: PaneKind::MemoryList,
        format_memory_addresses: BinaryFormat::Hex,
        format_memory: BinaryFormat::Decimal,
        format_register_values: BinaryFormat::Decimal,
        stack_pointer: 0,
        frame_pointer: 0,
        dynamic_link: 0,
        page_table_base: 0,
        interrupt_table_base: 0,
        steps_to_execute: 0,
    };

    loop {
        while state.steps_to_execute > 0 {
            let result = cpu.process(&mut memory);
            state.steps_to_execute -= 1;
            if result == crate::maple::cpu::ExecutionResult::Exit {
                break;
            }
        }

        sync_state_from_cpu(&mut state, &cpu);
        terminal.draw(|frame| render(frame, &mut state, &memory, &cpu))?;

        if let Event::Key(key) = event::read()? {
            if handle_input(key, &mut state) {
                break Ok(());
            }
        }
    }
}

fn sync_state_from_cpu(state: &mut AppState, cpu: &MapleCPU) {
    state.program_counter = cpu.get_program_counter();
    state.stack_pointer = cpu.get_stack_pointer();
    state.frame_pointer = cpu.get_frame_pointer();
    state.dynamic_link = cpu.get_dynamic_link();
    state.page_table_base = cpu.get_page_table_base();
    state.interrupt_table_base = cpu.get_interrupt_table_base();
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

fn render_content(frame: &mut Frame, area: Rect, state: &mut AppState, memory: &Memory, cpu: &MapleCPU) {
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

pub fn format_address(value: u64, format: &BinaryFormat) -> String {
    match format {
        BinaryFormat::Hex => format!("0x{:08X}", value),
        BinaryFormat::Decimal => format!("{:10}", value),
        BinaryFormat::Binary => format!("0b{:032b}", value),
    }
}

pub fn format_value(value: u64, format: &BinaryFormat) -> String {
    match format {
        BinaryFormat::Hex => format!("0x{:016X}", value),
        BinaryFormat::Decimal => format!("{}", value),
        BinaryFormat::Binary => format!("0b{:064b}", value),
    }
}