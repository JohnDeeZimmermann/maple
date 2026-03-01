use crate::maple::cpu::{ExecutionResult, MapleCPU, CPU};
use crate::maple::instructions::move_instructions::execute_move_instruction;
use crate::maple::interrupt_codes::INTERRUPT_CODE_INVALID_OPCODE;
use crate::maple::memory::Memory;
use crate::maple::utils::{extract_from_binary_left, extract_from_binary_right};

struct InstructionArguments {
    op_code: u8,
    options: u8,
    rdest: u8,
    arg1_raw: u32,
    arg2_raw: u32
}

const OP_CODE_NOP: u8 = 0;
const OP_CODE_MOVE: u8 = 1;

pub fn execute_instruction(
    cpu: &mut MapleCPU,
    memory: &mut Memory,
    instruction: u64,
) -> ExecutionResult {
    let op_code = (instruction >> 48) as u8; // First 8 bits from the left represent the op code

    // Move instructions have a different layout
    if op_code == OP_CODE_MOVE {
        execute_move_instruction(cpu, memory, instruction);
        return ExecutionResult::Ok
    }

    // Extracting standardized values
    let args = InstructionArguments {
        op_code,
        options: extract_from_binary_left(instruction, 4, 8) as u8,
        rdest: extract_from_binary_left(instruction, 4, 12) as u8,
        arg1_raw: extract_from_binary_right(instruction, 24, 24) as u32,
        arg2_raw: extract_from_binary_right(instruction, 24, 0) as u32
    };

    match op_code {
        OP_CODE_NOP => {
            // Do nothing
        },

        _ => {
            cpu.raise_interrupt(INTERRUPT_CODE_INVALID_OPCODE);
        }
    }

    ExecutionResult::Ok
}
