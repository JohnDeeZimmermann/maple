use crate::maple::cpu::{ExecutionResult, MapleCPU, CPU};
use crate::maple::instructions::move_instructions::execute_move_instruction;
use crate::maple::interrupt_codes::INTERRUPT_CODE_INVALID_OPCODE;
use crate::maple::memory::Memory;
use crate::maple::utils::{extract_from_binary_left, extract_from_binary_right};

pub struct InstructionArguments {
    pub op_code: u8,
    pub options: u8,
    pub rdest: u8,
    pub arg1_raw: u32,
    pub arg2_raw: u32
}

const OP_CODE_NOP: u8 = 0;
const OP_CODE_MOVE: u8 = 1;
const OP_CODE_ADD_INTEGER: u8 = 2;
const OP_CODE_SUBTRACT_INTEGER: u8 = 3;
const OP_CODE_MULTIPLY_INTEGER: u8 = 4;
const OP_CODE_DIVIDE_INTEGER: u8 = 5;

pub fn execute_instruction(
    cpu: &mut MapleCPU,
    memory: &mut Memory,
    instruction: u64,
) -> ExecutionResult {
    let op_code = extract_from_binary_left(instruction, 8, 0) as u8;

    // Move instructions have a different layout
    if op_code == OP_CODE_MOVE {
        execute_move_instruction(cpu, instruction);
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
