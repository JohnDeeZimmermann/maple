use crate::maple::cpu::{MapleCPU, CPU};
use crate::maple::memory::Memory;
use crate::maple::utils::{extract_from_binary_left, extract_from_binary_right, resolve_potential_register_argument_value};

pub fn execute_move_instruction(cpu: &mut MapleCPU, instruction: u64) {
    let opt: u64 = extract_from_binary_left(instruction, 1, 8);
    let rdest: u8 = extract_from_binary_left(instruction, 4, 9) as u8;
    let raw_value: u64 = extract_from_binary_right(instruction, 51, 0);
    let is_move_not = opt == 1;

    let actual_value: u64 = resolve_potential_register_argument_value(cpu, raw_value);

    if is_move_not {
        cpu.set_register(rdest, !actual_value);
    } else {
        cpu.set_register(rdest, actual_value);
    }
}
