#[path = "common/mod.rs"]
mod common;

#[path = "instructions/branch.rs"]
mod branch;
#[path = "instructions/branch_link.rs"]
mod branch_link;
#[path = "instructions/compare_float.rs"]
mod compare_float;
#[path = "instructions/compare_int.rs"]
mod compare_int;
#[path = "instructions/compare_results.rs"]
mod compare_results;
#[path = "instructions/conditional_branch.rs"]
mod conditional_branch;
#[path = "instructions/conditional_skip.rs"]
mod conditional_skip;
#[path = "instructions/encoding.rs"]
mod encoding;
#[path = "instructions/exit.rs"]
mod exit;
#[path = "instructions/integer_math.rs"]
mod integer_math;
#[path = "instructions/load_register.rs"]
mod load_register;
#[path = "instructions/logical_operations.rs"]
mod logical_operations;
#[path = "instructions/logical_shift.rs"]
mod logical_shift;
#[path = "instructions/move_instruction.rs"]
mod move_instruction;
#[path = "instructions/pop_push.rs"]
mod pop_push;
#[path = "instructions/return_from_interrupt.rs"]
mod return_from_interrupt;
#[path = "instructions/software_interrupt.rs"]
mod software_interrupt;
#[path = "instructions/store_memory.rs"]
mod store_memory;
