use std::path::Component::ParentDir;
use crate::maple::cpu::{MapleCPU, CPU};
use crate::maple::interrupt_codes::INTERRUPT_CODE_ILLEGAL_DIRECT_ARGUMENT;

pub fn extract_from_binary_left(value: u64, section_size: u32, section_left_offset: u64) -> u64 {
    let section_right_offset = 64_u64 - section_left_offset - section_size as u64;
    extract_from_binary_right(value, section_size, section_right_offset)
}

pub fn extract_from_binary_right(value: u64, section_size: u32, section_right_offset: u64) -> u64 {
    let mask = if section_size >= 64 {
        u64::MAX
    } else {
        (1_u64 << section_size) - 1
    };

    (value >> section_right_offset) & mask
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

pub fn place_value_in_binary_from_right(value: u64, position: u8, slice_size: u8) -> u64 {

    let position = 64 - position - slice_size;

    if (position > 64) {
        panic!("Position provided may not be smaller than 0.")
    }

    place_value_in_binary_from_left(value, position, slice_size)
}

pub fn place_value_in_binary_from_left(value: u64, position: u8, slice_size: u8) -> u64 {
    (value & (2_u8.pow(slice_size as u32) as u64)) << position
}
