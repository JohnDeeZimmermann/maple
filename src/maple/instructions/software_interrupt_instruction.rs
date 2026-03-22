use crate::maple::cpu::MapleCPU;
use crate::maple::instructions::instructions::InstructionArguments;
use crate::maple::utils::resolve_potential_register_argument_value;

pub fn execute_software_interrupt_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let interrupt_code = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64);
    cpu.raise_interrupt(interrupt_code as u16);
}
