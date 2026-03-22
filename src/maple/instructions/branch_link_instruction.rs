use crate::maple::cpu::{MapleCPU, CPU};
use crate::maple::instructions::instructions::{
    safely_update_program_counter, InstructionArguments,
};
use crate::maple::memory::Memory;
use crate::maple::utils::resolve_potential_register_argument_value;

pub fn execute_branch_link_instruction(
    cpu: &mut MapleCPU,
    memory: &mut Memory,
    args: &InstructionArguments,
) {
    let offset = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64);
    let base_address = cpu.get_register(args.rdest);
    let target_address = base_address + offset;
    let link = cpu.get_program_counter() + 1;

    if safely_update_program_counter(cpu, memory, target_address as u32) {
        cpu.set_dynamic_link(link);
    }
}
