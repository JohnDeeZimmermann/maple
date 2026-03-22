mod common;

use common::{
    encode_basic_instruction, encode_direct_argument, encode_register_argument,
    execute_single_instruction, new_cpu_and_memory, OP_CODE_LOGICAL_OPERATIONS,
};
use maple::maple::cpu::CPU;

fn and_instruction(rdest: u8, arg1_raw: u32, arg2_raw: u32) -> u64 {
    encode_basic_instruction(OP_CODE_LOGICAL_OPERATIONS, rdest, arg1_raw, arg2_raw)
}

fn create_logical_op_instruction(options: u8, rdest: u8, arg1_raw: u32, arg2_raw: u32) -> u64 {
    maple::maple::instructions::instructions::create_basic_instruction(
        maple::maple::instructions::instructions::InstructionArguments {
            op_code: OP_CODE_LOGICAL_OPERATIONS,
            options,
            rdest,
            arg1_raw,
            arg2_raw,
        },
    )
}

#[test]
fn and_direct_operands_produces_expected_result() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = and_instruction(
        0,
        encode_direct_argument(0b1100),
        encode_direct_argument(0b1010),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0b1000);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn orr_direct_operands_produces_expected_result() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = create_logical_op_instruction(
        1,
        0,
        encode_direct_argument(0b1100),
        encode_direct_argument(0b1010),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0b1110);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn xor_direct_operands_produces_expected_result() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = create_logical_op_instruction(
        2,
        0,
        encode_direct_argument(0b1100),
        encode_direct_argument(0b1010),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0b0110);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn and_register_operands_produces_expected_result() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b1100);
    cpu.set_register(2, 0b1010);
    let instruction = and_instruction(3, encode_register_argument(1), encode_register_argument(2));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(3), 0b1000);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn orr_register_operands_produces_expected_result() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b1100);
    cpu.set_register(2, 0b1010);
    let instruction = create_logical_op_instruction(
        1,
        3,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(3), 0b1110);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn xor_register_operands_produces_expected_result() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b1100);
    cpu.set_register(2, 0b1010);
    let instruction = create_logical_op_instruction(
        2,
        3,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(3), 0b0110);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn and_mixed_operands_produces_expected_result() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b11110000);
    let instruction = and_instruction(
        2,
        encode_register_argument(1),
        encode_direct_argument(0b10101010),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(2), 0b10100000);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn orr_mixed_operands_produces_expected_result() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b11110000);
    let instruction = create_logical_op_instruction(
        1,
        2,
        encode_register_argument(1),
        encode_direct_argument(0b10101010),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(2), 0b11111010);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn xor_mixed_operands_produces_expected_result() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b11110000);
    let instruction = create_logical_op_instruction(
        2,
        2,
        encode_register_argument(1),
        encode_direct_argument(0b10101010),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(2), 0b01011010);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn and_same_register_produces_identity() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b10101010);
    let instruction = and_instruction(1, encode_register_argument(1), encode_register_argument(1));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(1), 0b10101010);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn xor_same_register_produces_zero() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b10101010);
    let instruction = create_logical_op_instruction(
        2,
        1,
        encode_register_argument(1),
        encode_register_argument(1),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(1), 0);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn and_with_zeros_produces_zero() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = and_instruction(
        0,
        encode_direct_argument(0b1100),
        encode_direct_argument(0b0011),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn orr_with_zeros_produces_other() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = create_logical_op_instruction(
        1,
        0,
        encode_direct_argument(0),
        encode_direct_argument(0b1010),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0b1010);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn xor_with_zeros_produces_other() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = create_logical_op_instruction(
        2,
        0,
        encode_direct_argument(0),
        encode_direct_argument(0b1010),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0b1010);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn and_all_ones_produces_other() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0xFFFF_FFFF_FFFF_FFFF);
    let instruction = and_instruction(
        0,
        encode_register_argument(1),
        encode_direct_argument(0b1010),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0b1010);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn orr_all_ones_produces_all_ones() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0xFFFF_FFFF_FFFF_FFFF);
    let instruction = create_logical_op_instruction(
        1,
        0,
        encode_register_argument(1),
        encode_direct_argument(0b1010),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xFFFF_FFFF_FFFF_FFFF);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn xor_all_ones_produces_bitwise_not_of_other() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0xFFFF_FFFF_FFFF_FFFF);
    let instruction = create_logical_op_instruction(
        2,
        0,
        encode_register_argument(1),
        encode_direct_argument(0b1010),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), !0b1010_u64);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn logical_operations_unknown_option_returns_zero() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = create_logical_op_instruction(
        99,
        0,
        encode_direct_argument(42),
        encode_direct_argument(15),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn and_large_values() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0xFF00FF00);
    cpu.set_register(2, 0x0F0F0F0F);
    let instruction = and_instruction(0, encode_register_argument(1), encode_register_argument(2));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0x0F000F00);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn orr_large_values() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0xFF00FF00);
    cpu.set_register(2, 0x0F0F0F0F);
    let instruction = create_logical_op_instruction(
        1,
        0,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xFF0FFF0F);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn xor_large_values() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0xFF00FF00);
    cpu.set_register(2, 0x0F0F0F0F);
    let instruction = create_logical_op_instruction(
        2,
        0,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xF00FF00F);
    assert_eq!(cpu.get_program_counter(), 1);
}
