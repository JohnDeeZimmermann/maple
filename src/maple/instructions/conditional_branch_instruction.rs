use crate::maple::cpu::{ExecutionMode, MapleCPU, CPU};
use crate::maple::instructions::instructions::{is_condition_option_met, safely_update_program_counter, InstructionArguments};
use crate::maple::memory::Memory;
use crate::maple::utils::{get_conditional_result, resolve_potential_register_argument_value};

pub fn execute_conditional_branch_instruction(cpu: &mut MapleCPU, memory: &mut Memory, args: &InstructionArguments) {
    let result = get_conditional_result(cpu);
    let offset = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64);
    let base_address = cpu.get_register(args.rdest);
    let target_address =  base_address + offset;

    if !is_condition_option_met(args.options, result) {
        return;
    };

    safely_update_program_counter(cpu, memory, target_address as u32);
}
