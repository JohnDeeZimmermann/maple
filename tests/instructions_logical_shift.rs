mod common;

use common::{
    encode_basic_instruction, encode_direct_argument, encode_register_argument,
    execute_single_instruction, new_cpu_and_memory, OP_CODE_LOGICAL_SHIFT,
};

fn lsl_instruction(rdest: u8, arg1_raw: u32, arg2_raw: u32) -> u64 {
    encode_basic_instruction(OP_CODE_LOGICAL_SHIFT, rdest, arg1_raw, arg2_raw)
}

fn lsr_instruction(rdest: u8, arg1_raw: u32, arg2_raw: u32) -> u64 {
    create_lsl_instruction_with_options(OP_CODE_LOGICAL_SHIFT, 1, rdest, arg1_raw, arg2_raw)
}

fn create_lsl_instruction_with_options(
    op_code: u8,
    options: u8,
    rdest: u8,
    arg1_raw: u32,
    arg2_raw: u32,
) -> u64 {
    maple::maple::instructions::instructions::create_basic_instruction(
        maple::maple::instructions::instructions::InstructionArguments {
            op_code,
            options,
            rdest,
            arg1_raw,
            arg2_raw,
        },
    )
}

#[test]
fn lsl_with_direct_value_and_direct_shift() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = lsl_instruction(0, encode_direct_argument(0b1010), encode_direct_argument(2));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0b101000);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsr_with_direct_value_and_direct_shift() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = lsr_instruction(
        0,
        encode_direct_argument(0b101000),
        encode_direct_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0b1010);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsl_with_register_value_and_register_shift() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b11110000);
    cpu.set_register(2, 4);
    let instruction = lsl_instruction(3, encode_register_argument(1), encode_register_argument(2));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(3), 0b111100000000);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsr_with_register_value_and_register_shift() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b111100000000);
    cpu.set_register(2, 4);
    let instruction = lsr_instruction(3, encode_register_argument(1), encode_register_argument(2));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(3), 0b11110000);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsl_zero_shift_returns_original_value() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = lsl_instruction(0, encode_direct_argument(0xFF), encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xFF);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsr_zero_shift_returns_original_value() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = lsr_instruction(0, encode_direct_argument(0xFF00), encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xFF00);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsl_with_mixed_arguments() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b1);
    let instruction = lsl_instruction(2, encode_register_argument(1), encode_direct_argument(63));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(2), 0b1 << 63);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsr_with_mixed_arguments() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 1_u64 << 63);
    let instruction = lsr_instruction(2, encode_register_argument(1), encode_direct_argument(63));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(2), 1);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn logical_shift_unknown_option_returns_zero() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = create_lsl_instruction_with_options(
        OP_CODE_LOGICAL_SHIFT,
        99,
        0,
        encode_direct_argument(42),
        encode_direct_argument(5),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsl_with_max_shift_of_63_works() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 1_u64);
    let instruction = lsl_instruction(0, encode_register_argument(1), encode_direct_argument(63));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 1_u64 << 63);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsr_with_max_shift_of_63_works() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 1_u64 << 63);
    let instruction = lsr_instruction(0, encode_register_argument(1), encode_direct_argument(63));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 1);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsl_result_stored_in_destination_register() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(2, 0b101);
    let instruction = lsl_instruction(2, encode_register_argument(2), encode_direct_argument(1));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(2), 0b1010);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsr_result_stored_in_destination_register() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(2, 0b1010);
    let instruction = lsr_instruction(2, encode_register_argument(2), encode_direct_argument(1));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(2), 0b101);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsl_shift_by_64_returns_zero() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 1_u64);
    let instruction = lsl_instruction(0, encode_register_argument(1), encode_direct_argument(64));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsr_shift_by_64_returns_zero() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 1_u64 << 63);
    let instruction = lsr_instruction(0, encode_register_argument(1), encode_direct_argument(64));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsl_shift_by_larger_than_64_returns_zero() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0b1);
    let instruction = lsl_instruction(0, encode_register_argument(1), encode_direct_argument(100));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn lsr_shift_by_larger_than_64_returns_zero() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 1_u64 << 63);
    let instruction = lsr_instruction(0, encode_register_argument(1), encode_direct_argument(100));

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0);
    assert_eq!(cpu.get_program_counter(), 1);
}
