use crate::maple::cpu::MapleCPU;
use crate::maple::instructions::instructions::{is_condition_option_met, InstructionArguments};
use crate::maple::utils::get_conditional_result;

pub fn execute_compare_results_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let result = get_conditional_result(cpu);
    let condition_met = is_condition_option_met(args.options, result);
    cpu.set_register(args.rdest, condition_met as u64);
}
