use crate::maple::{cpu::MapleCPU, instructions::instructions::{InstructionArguments, perform_int_compare}, utils::resolve_potential_register_argument_value};

pub fn execute_compare_int_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let a = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64);
    let b = resolve_potential_register_argument_value(cpu, args.arg2_raw as u64);

    perform_int_compare(cpu, a as i64, b as i64); // Doesn't do anything but to perform the comparison
}
