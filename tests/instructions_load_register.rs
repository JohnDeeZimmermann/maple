mod common;

use common::{
    encode_basic_instruction, encode_direct_argument, encode_register_argument,
    execute_single_instruction, new_cpu_and_memory, OP_CODE_LOAD_REGISTER,
};

fn ldr_instruction(rdest: u8, arg1_raw: u32, arg2_raw: u32) -> u64 {
    encode_basic_instruction(OP_CODE_LOAD_REGISTER, rdest, arg1_raw, arg2_raw)
}

#[test]
fn ldr_with_direct_base_and_direct_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    memory.write(10, 0xDEADBEEF, &mut cpu);
    let instruction = ldr_instruction(0, encode_direct_argument(10), encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xDEADBEEF);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn ldr_with_register_base_and_direct_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 20);
    memory.write(25, 0x12345678, &mut cpu);
    let instruction = ldr_instruction(0, encode_register_argument(1), encode_direct_argument(5));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0x12345678);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn ldr_with_direct_base_and_register_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(2, 7);
    memory.write(15, 0xABCDEF, &mut cpu);
    let instruction = ldr_instruction(0, encode_direct_argument(8), encode_register_argument(2));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xABCDEF);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn ldr_with_register_base_and_register_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 30);
    cpu.set_register(2, 12);
    memory.write(42, 0xFACECAFE, &mut cpu);
    let instruction = ldr_instruction(0, encode_register_argument(1), encode_register_argument(2));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xFACECAFE);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn ldr_stores_to_correct_destination_register() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(3, 5);
    memory.write(8, 0x11111111, &mut cpu);
    let instruction = ldr_instruction(3, encode_register_argument(3), encode_direct_argument(3));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(3), 0x11111111);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn ldr_with_zero_offset_returns_value_at_base() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 50);
    memory.write(50, 0x55555555, &mut cpu);
    let instruction = ldr_instruction(0, encode_register_argument(1), encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0x55555555);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn ldr_with_different_base_and_offset_reads_correctly() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    memory.write(50, 0x123456789ABCDEF0, &mut cpu);
    let instruction = ldr_instruction(0, encode_direct_argument(50), encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0x123456789ABCDEF0);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn ldr_does_not_modify_other_registers() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 100);
    cpu.set_register(2, 200);
    cpu.set_register(4, 400);
    memory.write(150, 0x9999, &mut cpu);
    let instruction = ldr_instruction(3, encode_register_argument(1), encode_direct_argument(50));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(1), 100);
    assert_eq!(cpu.get_register(2), 200);
    assert_eq!(cpu.get_register(3), 0x9999);
    assert_eq!(cpu.get_register(4), 400);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn ldr_with_large_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(0, 0);
    memory.write(200, 0xCAFEBABE, &mut cpu);
    let instruction = ldr_instruction(1, encode_register_argument(0), encode_direct_argument(200));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(1), 0xCAFEBABE);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn ldr_with_offset_reads_correct_value() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 100);
    memory.write(100, 0xABCD, &mut cpu);
    let instruction = ldr_instruction(0, encode_register_argument(1), encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xABCD);
    assert_eq!(cpu.get_program_counter(), 1);
}
