use crate::maple::cpu::{MapleCPU, CPU};
use crate::maple::instructions::instructions::InstructionArguments;
use crate::maple::utils::get_conditional_result;

pub fn execute_compare_results_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let result = get_conditional_result(cpu);
    cpu.set_register(args.rdest, result.zero as u64);
}
