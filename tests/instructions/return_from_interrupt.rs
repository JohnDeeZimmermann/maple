use crate::common::{
    encode_basic_instruction, execute_single_instruction, new_cpu_and_memory,
    OP_CODE_RETURN_FROM_INTERRUPT,
};
use maple::maple::cpu::{ExecutionMode, ExecutionResult};

fn rfi_instruction() -> u64 {
    encode_basic_instruction(OP_CODE_RETURN_FROM_INTERRUPT, 0, 0, 0)
}

#[test]
fn rfi_restores_old_program_counter() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_system_info(0x1234);
    let instruction = rfi_instruction();

    let result = execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert!(result == ExecutionResult::Ok);
    assert_eq!(cpu.get_program_counter(), 0x1234);
}

#[test]
fn rfi_switches_to_user_mode() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.mode = ExecutionMode::Kernel;
    cpu.set_system_info(0x5678);
    let instruction = rfi_instruction();

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert!(cpu.mode == ExecutionMode::User);
}

#[test]
fn rfi_does_not_modify_general_registers() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(0, 0xDEAD);
    cpu.set_register(1, 0xBEEF);
    cpu.set_register(2, 0xCAFE);
    cpu.set_system_info(0x9ABC);
    let instruction = rfi_instruction();

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xDEAD);
    assert_eq!(cpu.get_register(1), 0xBEEF);
    assert_eq!(cpu.get_register(2), 0xCAFE);
}
