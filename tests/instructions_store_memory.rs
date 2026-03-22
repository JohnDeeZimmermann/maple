mod common;

use common::{
    encode_basic_instruction, encode_direct_argument, encode_register_argument,
    execute_single_instruction, new_cpu_and_memory, OP_CODE_STORE_MEMORY,
};

fn str_instruction(rdest: u8, arg1_raw: u32, arg2_raw: u32) -> u64 {
    encode_basic_instruction(OP_CODE_STORE_MEMORY, rdest, arg1_raw, arg2_raw)
}

#[test]
fn str_with_direct_base_and_direct_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(0, 0xDEADBEEF);
    let instruction = str_instruction(0, encode_direct_argument(10), encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(10, &mut cpu), 0xDEADBEEF);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn str_with_register_base_and_direct_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 20);
    cpu.set_register(0, 0x12345678);
    let instruction = str_instruction(0, encode_register_argument(1), encode_direct_argument(5));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(25, &mut cpu), 0x12345678);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn str_with_direct_base_and_register_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(2, 7);
    cpu.set_register(0, 0xABCDEF);
    let instruction = str_instruction(0, encode_direct_argument(8), encode_register_argument(2));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(15, &mut cpu), 0xABCDEF);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn str_with_register_base_and_register_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 30);
    cpu.set_register(2, 12);
    cpu.set_register(0, 0xFACECAFE);
    let instruction = str_instruction(0, encode_register_argument(1), encode_register_argument(2));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(42, &mut cpu), 0xFACECAFE);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn str_value_comes_from_rdest_register() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(3, 0x11111111);
    let instruction = str_instruction(3, encode_direct_argument(50), encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(50, &mut cpu), 0x11111111);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn str_does_not_modify_cpu_registers() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(0, 0x9999);
    cpu.set_register(1, 100);
    cpu.set_register(2, 200);
    cpu.set_register(4, 400);
    let instruction = str_instruction(0, encode_register_argument(1), encode_direct_argument(50));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(150, &mut cpu), 0x9999);
    assert_eq!(cpu.get_register(0), 0x9999);
    assert_eq!(cpu.get_register(1), 100);
    assert_eq!(cpu.get_register(2), 200);
    assert_eq!(cpu.get_register(4), 400);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn str_with_large_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0xCAFEBABE);
    let instruction = str_instruction(1, encode_direct_argument(0), encode_direct_argument(200));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(200, &mut cpu), 0xCAFEBABE);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn str_stores_zero_value() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(0, 0);
    let instruction = str_instruction(0, encode_direct_argument(99), encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(99, &mut cpu), 0);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn str_overwrites_existing_memory_value() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    memory.write(50, 0xAAAAAAAA, &mut cpu);
    cpu.set_register(0, 0xBBBBBBBB);
    let instruction = str_instruction(0, encode_direct_argument(50), encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(50, &mut cpu), 0xBBBBBBBB);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn str_different_register_sources_work() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(5, 0x123456);
    let instruction = str_instruction(5, encode_direct_argument(30), encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(30, &mut cpu), 0x123456);
    assert_eq!(cpu.get_program_counter(), 1);
}
