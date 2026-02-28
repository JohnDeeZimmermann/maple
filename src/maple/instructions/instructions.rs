use crate::maple::cpu::{ExecutionResult, MapleCPU};
use crate::maple::memory::Memory;

const OP_CODE_NOP: u8 = 0;

pub fn execute_instruction(cpu: &mut MapleCPU, memory: &mut Memory, instruction: u64) -> ExecutionResult {
    let op_code = (instruction >> 48) as u8; // First 8 bits from the left represent the op code

    if op_code == OP_CODE_NOP {
        return ExecutionResult::Ok;
    }
    
    

    ExecutionResult::Ok
}