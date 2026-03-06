use std::path::Component::ParentDir;
use crate::maple::cpu::{MapleCPU, CPU};
use crate::maple::interrupt_codes::INTERRUPT_CODE_ILLEGAL_DIRECT_ARGUMENT;

pub fn extract_from_binary_left(value: u64, section_size: u32, section_left_offset: u64) -> u64 {
    extract_from_binary_right(value, section_size, 64 - section_left_offset + 1)
}

pub fn extract_from_binary_right(value: u64, section_size: u32, section_right_offset: u64) -> u64 {
    (value >> section_right_offset) & (2_i32.pow(section_size - 1) as u64)
}

pub fn resolve_potential_register_argument_value(cpu: &MapleCPU, argument: u64) -> u64 {
    let is_register = (argument & 1) == 1;
    if is_register {
        let reg_num = extract_from_binary_right(argument, 4, 1) as u8;
        cpu.get_register(reg_num)
    } else {
        argument >> 1
    }
}

pub fn resolve_required_register_argument_value(cpu: &mut MapleCPU, argument: u64) -> u64 {
    let is_register = (argument & 1) == 1;
    if !is_register {
        cpu.raise_interrupt(INTERRUPT_CODE_ILLEGAL_DIRECT_ARGUMENT);
        return 0
    }
    let reg_num = extract_from_binary_right(argument, 4, 1) as u8;
    cpu.get_register(reg_num)
}