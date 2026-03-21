use crate::maple::cpu::{ExecutionResult, MapleCPU, CPU};
use crate::maple::instructions::conditional_skip_instruction::execute_conditional_skip_instruction;
use crate::maple::instructions::integer_math_instructions::{execute_add_integer_instruction, execute_divide_integer_instruction, execute_multiply_integer_instruction, execute_subtract_integer_instruction, update_conditional_result_register};
use crate::maple::instructions::move_instructions::execute_move_instruction;
use crate::maple::interrupt_codes::INTERRUPT_CODE_INVALID_OPCODE;
use crate::maple::memory::Memory;
use crate::maple::utils::{ConditionalResult, extract_from_binary_left, extract_from_binary_right};

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
const OP_CONDITIONAL_SKIP: u8 = 6;

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
        OP_CODE_ADD_INTEGER => {
            execute_add_integer_instruction(cpu, &args);
        },
        OP_CODE_SUBTRACT_INTEGER => {
            execute_subtract_integer_instruction(cpu, &args);
        },
        OP_CODE_MULTIPLY_INTEGER => {
            execute_multiply_integer_instruction(cpu, &args);
        },
        OP_CODE_DIVIDE_INTEGER => {
            execute_divide_integer_instruction(cpu, &args);
        },
        OP_CODE_CONDITIONAL_SKIP => {
            execute_conditional_skip_instruction(cpu, &args);
        },
        _ => {
            cpu.raise_interrupt(INTERRUPT_CODE_INVALID_OPCODE);
        }
    }

    ExecutionResult::Ok
}

pub fn create_basic_instruction(args: InstructionArguments) -> u64 {
    let op_code = (args.op_code as u64 & 0xFF) << 56;
    let options = (args.options as u64 & 0xF) << 52;
    let rdest = (args.rdest as u64 & 0xF) << 48;
    let arg1 = (args.arg1_raw as u64 & 0xFF_FFFF) << 24;
    let arg2 = args.arg2_raw as u64 & 0xFF_FFFF;

    op_code | options | rdest | arg1 | arg2
}

pub fn perform_compare(cpu: &mut MapleCPU, a: i64, b: i64) {
    let (result, overflowed) = a.overflowing_sub(b);
    update_conditional_result_register(cpu, result, overflowed);
}

pub fn is_condition_option_met(options: u8, result: ConditionalResult) -> bool {
    match (options) {
        CONDITION_OPTION_EQ => result.zero,
        CONDITION_OPTION_NEQ => !result.zero,
        CONDITION_OPTION_GT => !result.zero && !result.negative,
        CONDITION_OPTION_LT => !result.zero && result.negative,
        CONDITION_OPTION_GTE => result.zero || !result.negative,
        CONDITION_OPTION_GTE => result.zero || !result.negative,
        _ => false
    }
}

