use crate::common::{
    configure_interrupt_table, encode_basic_instruction, encode_direct_argument,
    encode_register_argument, execute_single_instruction, new_cpu_and_memory,
    OP_CODE_SOFTWARE_INTERRUPT,
};
use maple::maple::cpu::{ExecutionMode, ExecutionResult};

fn swi_instruction(interrupt_code_raw: u32) -> u64 {
    encode_basic_instruction(OP_CODE_SOFTWARE_INTERRUPT, 0, interrupt_code_raw, 0)
}

#[test]
fn swi_with_valid_code_raises_interrupt() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    configure_interrupt_table(&mut cpu, 100, 10);
    let instruction = swi_instruction(encode_direct_argument(5));

    let result = execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert!(result == ExecutionResult::Ok);
    assert!(cpu.get_program_counter() == 105);
}

#[test]
fn swi_with_code_from_register() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    configure_interrupt_table(&mut cpu, 200, 20);
    cpu.set_register(2, 10);
    let instruction = encode_basic_instruction(
        OP_CODE_SOFTWARE_INTERRUPT,
        0,
        encode_register_argument(2),
        0,
    );

    let result = execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert!(result == ExecutionResult::Ok);
    assert!(cpu.get_program_counter() == 210);
}

#[test]
fn swi_switches_to_kernel_mode() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.mode = ExecutionMode::User;
    configure_interrupt_table(&mut cpu, 100, 10);
    let instruction = swi_instruction(encode_direct_argument(5));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert!(cpu.mode == ExecutionMode::Kernel);
}

#[test]
fn swi_stores_old_pc_in_system_info() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_program_counter(10);
    configure_interrupt_table(&mut cpu, 100, 10);
    let instruction = swi_instruction(encode_direct_argument(5));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let old_pc = cpu.get_old_program_counter();
    assert_eq!(old_pc, 10);
}

#[test]
fn swi_does_not_modify_general_registers() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(0, 0xDEAD);
    cpu.set_register(1, 0xBEEF);
    cpu.set_register(2, 0xCAFE);
    configure_interrupt_table(&mut cpu, 100, 10);
    let instruction = swi_instruction(encode_direct_argument(5));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xDEAD);
    assert_eq!(cpu.get_register(1), 0xBEEF);
    assert_eq!(cpu.get_register(2), 0xCAFE);
}
