use crate::maple::cpu::MapleCPU;
use crate::maple::instructions::instructions::InstructionArguments;
use crate::maple::memory::Memory;
use crate::maple::utils::resolve_potential_register_argument_value;

pub fn execute_store_memory_instruction(
    cpu: &mut MapleCPU,
    memory: &mut Memory,
    args: &InstructionArguments,
) {
    let base = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64);
    let offset = resolve_potential_register_argument_value(cpu, args.arg2_raw as u64);
    let address = base + offset;
    let value = cpu.get_register(args.rdest);

    memory.write(address as u32, value, cpu);
}
