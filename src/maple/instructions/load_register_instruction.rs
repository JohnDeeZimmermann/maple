use crate::maple::cpu::MapleCPU;
use crate::maple::instructions::instructions::InstructionArguments;
use crate::maple::memory::Memory;

pub fn execute_load_register_instruction(
    cpu: &mut MapleCPU,
    memory: &mut Memory,
    args: &InstructionArguments,
) {
}
