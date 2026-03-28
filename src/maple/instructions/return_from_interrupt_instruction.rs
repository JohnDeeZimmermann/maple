use crate::maple::cpu::{ExecutionMode, MapleCPU};
use crate::maple::instructions::instructions::InstructionArguments;

pub fn execute_return_from_interrupt_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let address = cpu.get_old_program_counter();

    cpu.mode = ExecutionMode::User;
    cpu.set_program_counter(address);
}
