use crate::maple::cpu::{CPU, MapleCPU};
use crate::maple::instructions::instructions::{InstructionArguments, safely_update_program_counter};
use crate::maple::memory::Memory;
use crate::maple::utils::{get_conditional_result, resolve_potential_register_argument_value};

pub fn execute(cpu: &mut MapleCPU, memory: &mut Memory, args: &InstructionArguments) {
    let offset = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64);
    let base_address = cpu.get_register(args.rdest);
    let target_address =  base_address + offset;

    safely_update_program_counter(cpu, memory, target_address as u32);
}
