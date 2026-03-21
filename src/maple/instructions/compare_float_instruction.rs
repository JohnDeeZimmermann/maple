use crate::maple::{cpu::MapleCPU, instructions::instructions::{InstructionArguments, perform_float_compare}, utils::resolve_potential_register_argument_value};

pub fn execute_compare_float_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let a = f64::from_bits(resolve_potential_register_argument_value(cpu, args.arg1_raw as u64));
    let b = f64::from_bits(resolve_potential_register_argument_value(cpu, args.arg2_raw as u64));

    perform_float_compare(cpu, a, b); // Doesn't do anything but to perform the comparison
}
