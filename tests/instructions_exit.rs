mod common;

use common::{
    encode_basic_instruction, execute_single_instruction, new_cpu_and_memory, OP_CODE_EXIT,
};
use maple::maple::cpu::{ExecutionMode, ExecutionResult, CPU};

fn exit_instruction() -> u64 {
    encode_basic_instruction(OP_CODE_EXIT, 0, 0, 0)
}

#[test]
fn exit_in_kernel_mode_returns_exit_result() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = exit_instruction();

    let result = execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert!(result == ExecutionResult::Exit);
}

#[test]
fn exit_in_kernel_mode_does_not_modify_registers() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(0, 0xDEAD);
    cpu.set_register(1, 0xBEEF);
    cpu.set_register(2, 0xCAFE);
    let instruction = exit_instruction();

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xDEAD);
    assert_eq!(cpu.get_register(1), 0xBEEF);
    assert_eq!(cpu.get_register(2), 0xCAFE);
}

#[test]
fn exit_in_user_mode_raises_interrupt() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.mode = ExecutionMode::User;
    let instruction = exit_instruction();

    let result = execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert!(result == ExecutionResult::Ok);
    assert!(cpu.get_program_counter() > 0);
}
