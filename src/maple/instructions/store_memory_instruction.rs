use crate::maple::cpu::MapleCPU;
use crate::maple::instructions::instructions::InstructionArguments;
use crate::maple::memory::Memory;

pub fn execute_store_memory_instruction(
    cpu: &mut MapleCPU,
    memory: &mut Memory,
    args: &InstructionArguments,
) {
}
