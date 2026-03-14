use crate::maple::cpu::{MapleCPU, CPU};
use crate::maple::instructions::instructions::InstructionArguments;
use crate::maple::interrupt_codes::INTERRUPT_CODE_INVALID_DIVISION_BY_ZERO;
use crate::maple::utils::{extract_from_binary_left, resolve_potential_register_argument_value};

pub fn execute_add_integer_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64) as i64;
    let value_b = resolve_potential_register_argument_value(cpu, args.arg2_raw as u64) as i64;

    let (result, overflowed) = value_a.overflowing_add(value_b);

    cpu.set_register(args.rdest, result as u64);
    update_conditional_result_register(cpu, result, overflowed);
}

pub fn execute_subtract_integer_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64) as i64;
    let value_b = resolve_potential_register_argument_value(cpu, args.arg2_raw as u64) as i64;

    let (result, overflowed) = value_a.overflowing_sub(value_b);

    cpu.set_register(args.rdest, result as u64);
    update_conditional_result_register(cpu, result, overflowed);
}

pub fn execute_multiply_integer_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64) as i64;
    let value_b = resolve_potential_register_argument_value(cpu, args.arg2_raw as u64) as i64;

    let (result, overflowed) = value_a.overflowing_mul(value_b);

    cpu.set_register(args.rdest, result as u64);
    update_conditional_result_register(cpu, result, overflowed);
}

pub fn execute_divide_integer_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64) as i64;
    let value_b = resolve_potential_register_argument_value(cpu, args.arg2_raw as u64) as i64;

    if value_b == 0 {
        cpu.raise_interrupt(INTERRUPT_CODE_INVALID_DIVISION_BY_ZERO);
        return;
    }

    let (result, overflowed) = value_a.overflowing_div(value_b);

    cpu.set_register(args.rdest, result as u64);
    update_conditional_result_register(cpu, result, overflowed);
}

fn update_conditional_result_register(cpu: &mut MapleCPU, operation_result: i64, overflowed: bool) {
    let overflow = overflowed as u64;
    let zero = ((operation_result == 0) as u64) << 1;
    let negative = extract_from_binary_left(operation_result as u64, 1, 0) << 2; // MSB bit
    let parity = ((operation_result.count_ones() & 1 == 0) as u64) << 3;

    let current_register = cpu.get_result_register();
    let updated = (current_register & 0xFFFFFFFFFFFFFFF0) | parity | negative | zero | overflow;
    cpu.set_result_register(updated);
}
