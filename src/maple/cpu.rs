use crate::maple::instructions::instructions::execute_instruction;
use crate::maple::interrupt_codes::{
    INTERRUPT_CODE_ILLEGAL_REGISTER_MODIFICATION, INTERRUPT_CODE_INVALID_INTERRUPT_CODE,
};
use crate::maple::memory::Memory;
use crate::maple::utils::{extract_from_binary_left, place_value_in_binary_from_right};
use std::cmp::PartialEq;

#[derive(PartialEq)]
pub enum ExecutionMode {
    User,
    Kernel,
}

#[derive(PartialEq)]
pub enum ExecutionResult {
    Ok,
    Exit,
}

const REGISTER_STACK_POINTER: u8 = 6;
const REGISTER_PROGRAM_COUNTER: u8 = 7;
const REGISTER_DYNAMIC_LINK: u8 = 8;
const REGISTER_RESULT: u8 = 9;
const REGISTER_IO_POINTER: u8 = 10;
const REGISTER_TABLE_BASE: u8 = 11;
const REGISTER_SYSTEM_INFO: u8 = 12;
const REGISTER_FRAME_POINTER: u8 = 13;

pub struct MapleCPU {
    pub mode: ExecutionMode,
    pub registers: [u64; 16],
}

impl MapleCPU {
    pub fn new() -> Self {
        MapleCPU {
            registers: [0; 16],
            mode: ExecutionMode::Kernel,
        }
    }
}

impl MapleCPU {
    pub fn process(&mut self, memory: &mut Memory) -> ExecutionResult {
        let instruction = self.fetch(memory);
        self.decode_and_execute(memory, instruction)
    }

    pub fn fetch(&mut self, memory: &mut Memory) -> u64 {
        let address = self.get_program_counter() as u32;
        memory.read(address, self)
    }

    pub fn decode_and_execute(&mut self, memory: &mut Memory, instruction: u64) -> ExecutionResult {
        let initial_pc = self.get_program_counter();
        let result = execute_instruction(self, memory, instruction);

        if self.get_program_counter() == initial_pc {
            self.increment_program_counter();
        }

        result
    }

    pub fn raise_interrupt(&mut self, code: u16) {
        let interrupt_table_base = extract_from_binary_left(self.get_system_info(), 16, 0) as u16;
        let interrupt_table_size = extract_from_binary_left(self.get_system_info(), 16, 16) as u16;

        if code > interrupt_table_size {
            self.raise_interrupt(INTERRUPT_CODE_INVALID_INTERRUPT_CODE);
            return;
        }

        let result = interrupt_table_base + code;

        self.mode = ExecutionMode::Kernel;

        // OLD_PC
        place_value_in_binary_from_right(self.get_program_counter(), 0, 32);
        self.set_program_counter(result as u64);
    }

    pub fn get_stack_pointer(&self) -> u64 {
        self.get_register(REGISTER_STACK_POINTER)
    }

    pub fn set_stack_pointer(&mut self, value: u64) {
        self.set_register(REGISTER_STACK_POINTER, value);
    }

    pub fn get_program_counter(&self) -> u64 {
        self.get_register(REGISTER_PROGRAM_COUNTER)
    }

    pub fn set_program_counter(&mut self, value: u64) {
        self.set_register(REGISTER_PROGRAM_COUNTER, value);
    }

    pub fn increment_program_counter(&mut self) {
        self.set_program_counter(self.get_program_counter() + 1);
    }

    pub fn get_dynamic_link(&self) -> u64 {
        self.get_register(REGISTER_DYNAMIC_LINK)
    }

    pub fn set_dynamic_link(&mut self, value: u64) {
        self.set_register(REGISTER_DYNAMIC_LINK, value);
    }

    pub fn get_result_register(&self) -> u64 {
        self.get_register(REGISTER_RESULT)
    }

    pub fn set_result_register(&mut self, value: u64) {
        self.set_register(REGISTER_RESULT, value);
    }

    pub fn get_io_pointer(&self) -> u64 {
        self.get_register(REGISTER_IO_POINTER)
    }

    pub fn set_io_pointer(&mut self, value: u64) {
        self.set_register(REGISTER_IO_POINTER, value);
    }

    pub fn get_page_table_base(&self) -> u64 {
        self.get_register(REGISTER_TABLE_BASE)
    }

    pub fn set_table_base(&mut self, value: u64) {
        self.set_register(REGISTER_TABLE_BASE, value);
    }

    pub fn get_system_info(&self) -> u64 {
        self.get_register(REGISTER_SYSTEM_INFO)
    }

    pub fn set_system_info(&mut self, value: u64) {
        self.set_register(REGISTER_SYSTEM_INFO, value);
    }

    pub fn get_frame_pointer(&self) -> u64 {
        self.get_register(REGISTER_FRAME_POINTER)
    }

    pub fn set_frame_pointer(&mut self, value: u64) {
        self.set_register(REGISTER_FRAME_POINTER, value);
    }

    pub fn get_gp_register(&self, gp: u8) -> u64 {
        self.get_register(gp)
    }

    pub fn set_gp_register(&mut self, gp: u8, value: u64) {
        self.set_register(gp, value);
    }

    pub fn get_hw_register(&self, hw: u8) -> u64 {
        let hw_base = 14;
        self.get_register(hw_base + hw)
    }

    pub fn set_hw_register(&mut self, hw: u8, value: u64) {
        let hw_base = 14;
        self.set_register(hw_base + hw, value);
    }

    pub fn get_register(&self, register: u8) -> u64 {
        if register < 16 {
            return self.registers[register as usize];
        }
        0
    }

    pub fn set_register(&mut self, register: u8, value: u64) {
        if self.mode == ExecutionMode::User && (10..=12).contains(&register) {
            self.raise_interrupt(INTERRUPT_CODE_ILLEGAL_REGISTER_MODIFICATION);
            return;
        }

        if register < 16 {
            self.registers[register as usize] = value;
        }
    }
}
