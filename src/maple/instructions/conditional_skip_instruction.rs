use crate::maple::cpu::{CPU, MapleCPU};
use crate::maple::instructions::condition_options::{CONDITION_OPTION_EQ, CONDITION_OPTION_GT, CONDITION_OPTION_GTE, CONDITION_OPTION_LT, CONDITION_OPTION_NEQ};
use crate::maple::instructions::instructions::{InstructionArguments, is_condition_option_met, perform_compare};
use crate::maple::utils::{get_conditional_result, resolve_potential_register_argument_value};

pub fn execute_conditional_skip_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    
    let a = cpu.get_register(args.rdest);
    let b = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64);
    
    perform_compare(cpu, a as i64, b as i64);
    
    let result = get_conditional_result(cpu);
    if is_condition_option_met(args.options, result) {
        cpu.increment_program_counter();
        cpu.increment_program_counter();
    }
}