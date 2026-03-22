use crate::maple::cpu::{ExecutionMode, ExecutionResult, MapleCPU};
use crate::maple::interrupt_codes::INTERRUPT_CODE_ILLEGAL_EXIT;

pub fn execute_exit_instruction(cpu: &mut MapleCPU) -> ExecutionResult {
    if cpu.mode == ExecutionMode::User {
        cpu.raise_interrupt(INTERRUPT_CODE_ILLEGAL_EXIT);
        return ExecutionResult::Ok;
    }

    return ExecutionResult::Exit;
}
