use crate::maple::cpu::ExecutionResult;

pub mod instructions;
pub mod move_instructions;
pub mod integer_math_instructions;
pub mod float_math_instructions;
pub mod conditional_skip_instruction;
pub mod condition_options;
pub mod compare_int_instruction;
pub mod compare_float_instruction;
pub mod compare_results_instruction;
pub mod conditional_branch_instruction;