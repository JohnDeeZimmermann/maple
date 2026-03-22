use std::ops::Sub;

use crate::maple::cpu::{ExecutionMode, ExecutionResult, MapleCPU, CPU};
use crate::maple::instructions::compare_float_instruction::execute_compare_float_instruction;
use crate::maple::instructions::compare_int_instruction::execute_compare_int_instruction;
use crate::maple::instructions::compare_results_instruction::execute_compare_results_instruction;
use crate::maple::instructions::conditional_branch_instruction::execute_conditional_branch_instruction;
use crate::maple::instructions::conditional_skip_instruction::execute_conditional_skip_instruction;
use crate::maple::instructions::float_math_instructions::{
    execute_add_float_instruction, execute_divide_float_instruction,
    execute_multiply_float_instruction, execute_subtract_float_instruction,
    update_conditional_result_register_float,
};
use crate::maple::instructions::integer_math_instructions::{
    execute_add_integer_instruction, execute_divide_integer_instruction,
    execute_multiply_integer_instruction, execute_subtract_integer_instruction,
    update_conditional_result_register_int,
};
use crate::maple::instructions::move_instructions::execute_move_instruction;
use crate::maple::interrupt_codes::INTERRUPT_CODE_INVALID_OPCODE;
use crate::maple::memory::Memory;
use crate::maple::utils::{extract_from_binary_left, extract_from_binary_right, ConditionalResult};

pub struct InstructionArguments {
    pub op_code: u8,
    pub options: u8,
    pub rdest: u8,
    pub arg1_raw: u32,
    pub arg2_raw: u32,
}

const OP_CODE_NOP: u8 = 0;
const OP_CODE_MOVE: u8 = 1;
const OP_CODE_ADD_INTEGER: u8 = 2;
const OP_CODE_SUBTRACT_INTEGER: u8 = 3;
const OP_CODE_MULTIPLY_INTEGER: u8 = 4;
const OP_CODE_DIVIDE_INTEGER: u8 = 5;
const OP_CODE_ADD_FLOAT: u8 = 6;
const OP_CODE_SUBTRACT_FLOAT: u8 = 7;
const OP_CODE_MULTIPLY_FLOAT: u8 = 8;
const OP_CODE_DIVIDE_FLOAT: u8 = 9;
const OP_CODE_CONDITIONAL_SKIP: u8 = 10;
const OP_CODE_COMPARE_INTEGER: u8 = 11;
const OP_CODE_COMPARE_FLOAT: u8 = 12;
const OP_CODE_COMPARE_RESULTS: u8 = 13;
const OP_CODE_CONDITIONAL_BRANCH: u8 = 14;
const OP_CODE_BRANCH: u8 = 15;
const OP_CODE_BRANCH_LINK: u8 = 16;
const OP_CODE_LOGICAL_SHIFT: u8 = 17;
const OP_CODE_LOGICAL_OPERATIONS: u8 = 18;
const OP_CODE_LOAD_REGISTER: u8 = 19;
const OP_CODE_STORE_MEMORY: u8 = 20;
const OP_CODE_POP_PUSH: u8 = 21;
const OP_CODE_EXIT: u8 = 22;
const OP_CODE_IO_READ_WRITE: u8 = 23;
const OP_CODE_SOFTWARE_INTERRUPT: u8 = 24;
const OP_CODE_RETURN_FROM_INTERRUPT: u8 = 25;

pub fn execute_instruction(
    cpu: &mut MapleCPU,
    memory: &mut Memory,
    instruction: u64,
) -> ExecutionResult {
    let op_code = extract_from_binary_left(instruction, 8, 0) as u8;

    // Move instructions have a different layout
    if op_code == OP_CODE_MOVE {
        execute_move_instruction(cpu, instruction);
        return ExecutionResult::Ok;
    }

    // Extracting standardized values
    let args = InstructionArguments {
        op_code,
        options: extract_from_binary_left(instruction, 4, 8) as u8,
        rdest: extract_from_binary_left(instruction, 4, 12) as u8,
        arg1_raw: extract_from_binary_right(instruction, 24, 24) as u32,
        arg2_raw: extract_from_binary_right(instruction, 24, 0) as u32,
    };

    match op_code {
        OP_CODE_NOP => {
            // Do nothing
        }
        OP_CODE_ADD_INTEGER => {
            execute_add_integer_instruction(cpu, &args);
        }
        OP_CODE_SUBTRACT_INTEGER => {
            execute_subtract_integer_instruction(cpu, &args);
        }
        OP_CODE_MULTIPLY_INTEGER => {
            execute_multiply_integer_instruction(cpu, &args);
        }
        OP_CODE_DIVIDE_INTEGER => {
            execute_divide_integer_instruction(cpu, &args);
        }
        OP_CODE_ADD_FLOAT => {
            execute_add_float_instruction(cpu, &args);
        }
        OP_CODE_SUBTRACT_FLOAT => {
            execute_subtract_float_instruction(cpu, &args);
        }
        OP_CODE_MULTIPLY_FLOAT => {
            execute_multiply_float_instruction(cpu, &args);
        }
        OP_CODE_DIVIDE_FLOAT => {
            execute_divide_float_instruction(cpu, &args);
        }
        OP_CODE_CONDITIONAL_SKIP => {
            execute_conditional_skip_instruction(cpu, &args);
        }
        OP_CODE_COMPARE_INTEGER => {
            execute_compare_int_instruction(cpu, &args);
        }
        OP_CODE_COMPARE_FLOAT => {
            execute_compare_float_instruction(cpu, &args);
        }
        OP_CODE_COMPARE_RESULTS => {
            execute_compare_results_instruction(cpu, &args);
        }
        OP_CODE_CONDITIONAL_BRANCH => {
            execute_conditional_branch_instruction(cpu, memory, &args);
        }
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

pub fn perform_int_compare(cpu: &mut MapleCPU, a: i64, b: i64) {
    let (result, overflowed) = a.overflowing_sub(b);
    update_conditional_result_register_int(cpu, result, overflowed);
}

pub fn perform_float_compare(cpu: &mut MapleCPU, a: f64, b: f64) {
    let result = a - b;
    update_conditional_result_register_float(cpu, result, false);
}

pub fn is_condition_option_met(options: u8, result: ConditionalResult) -> bool {
    match options {
        0 => result.zero,
        1 => !result.zero,
        2 => !result.zero && !result.negative,
        3 => !result.zero && result.negative,
        4 => result.zero || !result.negative,
        5 => result.zero || result.negative,
        _ => false,
    }
}

pub fn safely_update_program_counter(cpu: &mut MapleCPU, memory: &mut Memory, destination: u32) {
    let actual_dest = if cpu.mode == ExecutionMode::User {
        let resolved =
            memory.virtual_to_physical(destination, cpu.get_page_table_base() as u32, cpu);
        if resolved == 0 {
            return;
        }
        resolved
    } else {
        destination
    };

    cpu.set_program_counter(actual_dest as u64);
}
