use crate::maple::cpu::MapleCPU;
use crate::maple::utils::{
    extract_from_binary_left, extract_from_binary_right, resolve_argument_value,
    DirectArgumentSignMode,
};

pub fn execute_move_instruction(cpu: &mut MapleCPU, instruction: u64) {
    let opt: u64 = extract_from_binary_left(instruction, 1, 8);
    let rdest: u8 = extract_from_binary_left(instruction, 4, 9) as u8;
    let sign: u64 = extract_from_binary_left(instruction, 1, 13);
    let raw_value: u64 = extract_from_binary_right(instruction, 50, 0);
    let is_move_not = opt == 1;

    let value_with_sign =
        resolve_argument_value(cpu, raw_value, DirectArgumentSignMode::Unsigned, Some(sign));

    if is_move_not {
        cpu.set_register(rdest, !value_with_sign);
    } else {
        cpu.set_register(rdest, value_with_sign);
    }
}
