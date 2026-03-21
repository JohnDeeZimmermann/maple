mod common;

use common::{
    cr_negative, cr_zero, encode_basic_instruction, encode_register_argument,
    execute_single_instruction, new_cpu_and_memory, OP_CODE_COMPARE_FLOAT,
};
use maple::maple::cpu::CPU;

#[test]
fn compare_float_equal_values_sets_zero_flag() {
    // CMPF with equal float values should set zero flag.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 3.14_f64.to_bits());
    cpu.set_register(2, 3.14_f64.to_bits());
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_FLOAT,
        0,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let cr = cpu.get_result_register();
    assert!(cr_zero(cr));
    assert!(!cr_negative(cr));
}

#[test]
fn compare_float_greater_than_clears_zero_and_negative() {
    // CMPF where a > b should clear zero and negative flags.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 10.5_f64.to_bits());
    cpu.set_register(2, 5.2_f64.to_bits());
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_FLOAT,
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
fn compare_float_less_than_sets_negative_flag() {
    // CMPF where a < b should set negative flag.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 2.5_f64.to_bits());
    cpu.set_register(2, 7.8_f64.to_bits());
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_FLOAT,
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
fn compare_float_negative_values() {
    // CMPF with negative float values.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, (-5.0_f64).to_bits());
    cpu.set_register(2, (-3.0_f64).to_bits());
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_FLOAT,
        0,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    // -5.0 < -3.0, so result should be negative
    let cr = cpu.get_result_register();
    assert!(cr_negative(cr));
    assert!(!cr_zero(cr));
}

#[test]
fn compare_float_zero_comparison() {
    // CMPF with zero value.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0.0_f64.to_bits());
    cpu.set_register(2, 0.0_f64.to_bits());
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_FLOAT,
        0,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let cr = cpu.get_result_register();
    assert!(cr_zero(cr));
    assert!(!cr_negative(cr));
}

#[test]
fn compare_float_does_not_modify_destination_register() {
    // CMPF should not modify the destination register (it only sets flags).
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 1.5_f64.to_bits());
    cpu.set_register(2, 3.5_f64.to_bits());
    cpu.set_register(4, 0xDEAD_BEEF);
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_FLOAT,
        4,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(4), 0xDEAD_BEEF);
}

#[test]
fn compare_float_mixed_positive_negative() {
    // CMPF with positive vs negative values.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, (-2.5_f64).to_bits());
    cpu.set_register(2, 1.5_f64.to_bits());
    let instruction = encode_basic_instruction(
        OP_CODE_COMPARE_FLOAT,
        0,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    // -2.5 < 1.5, so result should be negative
    let cr = cpu.get_result_register();
    assert!(cr_negative(cr));
    assert!(!cr_zero(cr));
}
