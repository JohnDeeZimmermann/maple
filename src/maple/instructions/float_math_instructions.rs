use crate::maple::cpu::MapleCPU;
use crate::maple::instructions::instructions::InstructionArguments;
use crate::maple::interrupt_codes::INTERRUPT_CODE_INVALID_DIVISION_BY_ZERO;
use crate::maple::utils::{extract_from_binary_left, resolve_required_register_argument_value};

pub fn execute_add_float_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = f64::from_bits(resolve_required_register_argument_value(
        cpu,
        args.arg1_raw as u64,
    ));
    let value_b = f64::from_bits(resolve_required_register_argument_value(
        cpu,
        args.arg2_raw as u64,
    ));

    let result = value_a + value_b;

    cpu.set_register(args.rdest, result.to_bits());
    update_conditional_result_register_float(cpu, result, false);
}

pub fn execute_subtract_float_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = f64::from_bits(resolve_required_register_argument_value(
        cpu,
        args.arg1_raw as u64,
    ));
    let value_b = f64::from_bits(resolve_required_register_argument_value(
        cpu,
        args.arg2_raw as u64,
    ));

    let result = value_a - value_b;

    cpu.set_register(args.rdest, result.to_bits());
    update_conditional_result_register_float(cpu, result, false);
}

pub fn execute_multiply_float_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = f64::from_bits(resolve_required_register_argument_value(
        cpu,
        args.arg1_raw as u64,
    ));
    let value_b = f64::from_bits(resolve_required_register_argument_value(
        cpu,
        args.arg2_raw as u64,
    ));

    let result = value_a * value_b;

    cpu.set_register(args.rdest, result.to_bits());
    update_conditional_result_register_float(cpu, result, false);
}

pub fn execute_divide_float_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = f64::from_bits(resolve_required_register_argument_value(
        cpu,
        args.arg1_raw as u64,
    ));
    let value_b = f64::from_bits(resolve_required_register_argument_value(
        cpu,
        args.arg2_raw as u64,
    ));

    if value_b == 0.0 {
        cpu.raise_interrupt(INTERRUPT_CODE_INVALID_DIVISION_BY_ZERO);
    }

    let result = value_a / value_b;

    cpu.set_register(args.rdest, result.to_bits());
    update_conditional_result_register_float(cpu, result, false);
}

pub fn update_conditional_result_register_float(
    cpu: &mut MapleCPU,
    operation_result: f64,
    overflowed: bool,
) {
    let overflow = overflowed as u64;
    let zero = ((operation_result == 0.0) as u64) << 1;
    let negative = extract_from_binary_left(operation_result.to_bits(), 1, 0) << 2; // MSB bit
    let parity = (((operation_result as u64).count_ones() & 1 == 0) as u64) << 3;

    let current_register = cpu.get_result_register();
    let updated = (current_register & 0xFFFFFFFFFFFFFFF0) | parity | negative | zero | overflow;
    cpu.set_result_register(updated);
}
