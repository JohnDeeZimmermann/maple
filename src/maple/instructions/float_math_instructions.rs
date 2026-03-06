use crate::maple::cpu::{MapleCPU, CPU};
use crate::maple::instructions::instructions::InstructionArguments;
use crate::maple::interrupt_codes::INTERRUPT_CODE_INVALID_DIVISION_BY_ZERO;
use crate::maple::utils::resolve_required_register_argument_value;

pub fn execute_add_float_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = f64::from_bits(resolve_required_register_argument_value(cpu, args.arg1_raw as u64));
    let value_b = f64::from_bits(resolve_required_register_argument_value(cpu, args.arg2_raw as u64));

    let result = value_a + value_b;

    cpu.set_register(args.rdest, result.to_bits());
}

pub fn execute_subtract_float_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = f64::from_bits(resolve_required_register_argument_value(cpu, args.arg1_raw as u64));
    let value_b = f64::from_bits(resolve_required_register_argument_value(cpu, args.arg2_raw as u64));

    let result = value_a - value_b;

    cpu.set_register(args.rdest, result.to_bits());
}

pub fn execute_multiply_float_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = f64::from_bits(resolve_required_register_argument_value(cpu, args.arg1_raw as u64));
    let value_b = f64::from_bits(resolve_required_register_argument_value(cpu, args.arg2_raw as u64));

    let result = value_a * value_b;

    cpu.set_register(args.rdest, result.to_bits());
}

pub fn execute_divide_float_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let value_a = f64::from_bits(resolve_required_register_argument_value(cpu, args.arg1_raw as u64));
    let value_b = f64::from_bits(resolve_required_register_argument_value(cpu, args.arg2_raw as u64));

    if value_b == 0.0 {
        cpu.raise_interrupt(INTERRUPT_CODE_INVALID_DIVISION_BY_ZERO);
    }

    let result = value_a / value_b;

    cpu.set_register(args.rdest, result.to_bits());
}
