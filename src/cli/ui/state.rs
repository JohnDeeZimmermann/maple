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

impl AppState {
    pub fn new(max_address: u32) -> Self {
        Self {
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
        }
    }

    pub fn sync_from_cpu(&mut self, cpu: &crate::maple::cpu::MapleCPU) {
        self.program_counter = cpu.get_program_counter();
        self.stack_pointer = cpu.get_stack_pointer();
        self.frame_pointer = cpu.get_frame_pointer();
        self.dynamic_link = cpu.get_dynamic_link();
        self.page_table_base = cpu.get_page_table_base();
        self.interrupt_table_base = cpu.get_interrupt_table_base();
    }
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
