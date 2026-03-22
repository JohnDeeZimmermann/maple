mod common;

use common::{
    cr_negative, cr_overflow, cr_parity, cr_zero, encode_basic_instruction, encode_direct_argument,
    encode_register_argument, execute_single_instruction, new_cpu_and_memory,
    OP_CODE_COMPARE_INTEGER,
};

#[test]
fn compare_int_direct_operands_equal_sets_zero_flag() {
    // CMPI with equal direct values should set zero and parity flags.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(42),
        encode_direct_argument(42),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let cr = cpu.get_result_register();
    assert!(cr_zero(cr));
    assert!(cr_parity(cr));
    assert!(!cr_negative(cr));
    assert!(!cr_overflow(cr));
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn compare_int_direct_operands_greater_than_clears_zero_and_negative() {
    // CMPI where a > b should clear zero and negative flags.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(10),
        encode_direct_argument(5),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let cr = cpu.get_result_register();
    assert!(!cr_zero(cr));
    assert!(!cr_negative(cr));
}

#[test]
fn compare_int_direct_operands_less_than_sets_negative_flag() {
    // CMPI where a < b should set negative flag.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(5),
        encode_direct_argument(10),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let cr = cpu.get_result_register();
    assert!(cr_negative(cr));
    assert!(!cr_zero(cr));
}

#[test]
fn compare_int_register_operands_are_resolved() {
    // CMPI with register arguments should resolve values correctly.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 100);
    cpu.set_register(2, 50);
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let cr = cpu.get_result_register();
    assert!(!cr_zero(cr));
    assert!(!cr_negative(cr));
}

#[test]
fn compare_int_mixed_direct_and_register_operands() {
    // CMPI with left register, right direct should work correctly.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 25);
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_register_argument(1),
        encode_direct_argument(25),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let cr = cpu.get_result_register();
    assert!(cr_zero(cr));
}

#[test]
fn compare_int_negative_values_sets_negative_flag() {
    // CMPI with negative values should set negative when a < b.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, (-10_i64) as u64);
    cpu.set_register(2, (-5_i64) as u64);
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let cr = cpu.get_result_register();
    assert!(cr_negative(cr));
    assert!(!cr_zero(cr));
}

#[test]
fn compare_int_does_not_modify_destination_register() {
    // CMPI should not modify the destination register (it only sets flags).
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(3, 0xDEAD_BEEF);
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        3,
        encode_direct_argument(5),
        encode_direct_argument(10),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(3), 0xDEAD_BEEF);
}
