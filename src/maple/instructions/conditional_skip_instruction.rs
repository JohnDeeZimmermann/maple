use crate::maple::cpu::MapleCPU;
use crate::maple::instructions::instructions::{
    is_condition_option_met, perform_int_compare, InstructionArguments,
};
use crate::maple::utils::{get_conditional_result, resolve_potential_register_argument_value};

pub fn execute_conditional_skip_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let a = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64);
    let b = resolve_potential_register_argument_value(cpu, args.arg2_raw as u64);

    perform_int_compare(cpu, a as i64, b as i64);

    let result = get_conditional_result(cpu);
    if is_condition_option_met(args.options, result) {
        cpu.increment_program_counter();
        cpu.increment_program_counter();
    }
}
