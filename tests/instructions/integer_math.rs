use crate::common::{
    configure_interrupt_table, cr_negative, cr_overflow, cr_parity, cr_zero,
    encode_basic_instruction, encode_direct_argument, encode_register_argument,
    encode_signed_direct_argument, execute_single_instruction, new_cpu_and_memory,
    OP_CODE_ADD_INTEGER, OP_CODE_DIVIDE_INTEGER, OP_CODE_MULTIPLY_INTEGER,
    OP_CODE_SUBTRACT_INTEGER,
};

#[test]
fn addi_direct_operands_produces_expected_result() {
    // Basic ADDI direct+direct path.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_basic_instruction(
        OP_CODE_ADD_INTEGER,
        0,
        encode_direct_argument(7),
        encode_direct_argument(5),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 12);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn subi_direct_operands_produces_expected_result() {
    // Basic SUBI direct+direct path with negative result.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_basic_instruction(
        OP_CODE_SUBTRACT_INTEGER,
        1,
        encode_direct_argument(7),
        encode_direct_argument(12),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(1), (-5_i64) as u64);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn muli_direct_operands_produces_expected_result() {
    // Basic MULI direct+direct path.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_basic_instruction(
        OP_CODE_MULTIPLY_INTEGER,
        2,
        encode_direct_argument(6),
        encode_direct_argument(7),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(2), 42);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn divi_direct_operands_produces_expected_result() {
    // Basic DIVI direct+direct path.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_basic_instruction(
        OP_CODE_DIVIDE_INTEGER,
        3,
        encode_direct_argument(42),
        encode_direct_argument(6),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(3), 7);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn register_and_mixed_operands_are_resolved_for_all_integer_math_operations() {
    // First pass: both operands come from registers.
    let register_cases = [
        (OP_CODE_ADD_INTEGER, 10_i64, 7_i64, 17_i64),
        (OP_CODE_SUBTRACT_INTEGER, 10_i64, 7_i64, 3_i64),
        (OP_CODE_MULTIPLY_INTEGER, 10_i64, 7_i64, 70_i64),
        (OP_CODE_DIVIDE_INTEGER, 21_i64, 7_i64, 3_i64),
    ];

    for (op_code, a, b, expected) in register_cases {
        let (mut cpu, mut memory) = new_cpu_and_memory();
        cpu.set_register(1, a as u64);
        cpu.set_register(2, b as u64);
        let instruction = encode_basic_instruction(
            op_code,
            4,
            encode_register_argument(1),
            encode_register_argument(2),
        );

        execute_single_instruction(&mut cpu, &mut memory, instruction);

        assert_eq!(cpu.get_register(4), expected as u64);
        assert_eq!(cpu.get_program_counter(), 1);
    }

    // Second pass: left operand from register, right operand direct immediate.
    let mixed_cases = [
        (OP_CODE_ADD_INTEGER, 8_i64, 5_u32, 13_i64),
        (OP_CODE_SUBTRACT_INTEGER, 8_i64, 5_u32, 3_i64),
        (OP_CODE_MULTIPLY_INTEGER, 8_i64, 5_u32, 40_i64),
        (OP_CODE_DIVIDE_INTEGER, 20_i64, 5_u32, 4_i64),
    ];

    for (op_code, reg_value, direct_value, expected) in mixed_cases {
        let (mut cpu, mut memory) = new_cpu_and_memory();
        cpu.set_register(1, reg_value as u64);
        let instruction = encode_basic_instruction(
            op_code,
            5,
            encode_register_argument(1),
            encode_direct_argument(direct_value),
        );

        execute_single_instruction(&mut cpu, &mut memory, instruction);

        assert_eq!(cpu.get_register(5), expected as u64);
        assert_eq!(cpu.get_program_counter(), 1);
    }
}

#[test]
fn signed_direct_operands_are_supported_for_integer_math() {
    let cases = [
        (OP_CODE_ADD_INTEGER, -7_i32, 5_i32, -2_i64),
        (OP_CODE_SUBTRACT_INTEGER, -7_i32, -5_i32, -2_i64),
        (OP_CODE_MULTIPLY_INTEGER, -7_i32, 5_i32, -35_i64),
        (OP_CODE_DIVIDE_INTEGER, -21_i32, 7_i32, -3_i64),
    ];

    for (op_code, a, b, expected) in cases {
        let (mut cpu, mut memory) = new_cpu_and_memory();
        let instruction = encode_basic_instruction(
            op_code,
            2,
            encode_signed_direct_argument(a),
            encode_signed_direct_argument(b),
        );

        execute_single_instruction(&mut cpu, &mut memory, instruction);

        assert_eq!(cpu.get_register(2), expected as u64);
        assert_eq!(cpu.get_program_counter(), 1);
    }
}

#[test]
fn signed_direct_operands_can_mix_with_register_operands() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, (-9_i64) as u64);
    let instruction = encode_basic_instruction(
        OP_CODE_ADD_INTEGER,
        3,
        encode_register_argument(1),
        encode_signed_direct_argument(4),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(3), (-5_i64) as u64);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn signed_direct_arguments_sign_extend_to_the_supported_23_bit_range() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_basic_instruction(
        OP_CODE_ADD_INTEGER,
        0,
        encode_signed_direct_argument(-4_194_304),
        encode_signed_direct_argument(1),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), (-4_194_303_i64) as u64);
    assert!(cr_negative(cpu.get_result_register()));
}

#[test]
fn cr_zero_and_parity_flags_are_set_for_zero_result() {
    // 42 - 42 => zero result; zero+parity expected, no negative/overflow.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 42);
    let instruction = encode_basic_instruction(
        OP_CODE_SUBTRACT_INTEGER,
        0,
        encode_register_argument(1),
        encode_register_argument(1),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let cr = cpu.get_result_register();
    assert!(cr_zero(cr));
    assert!(cr_parity(cr));
    assert!(!cr_negative(cr));
    assert!(!cr_overflow(cr));
}

#[test]
fn cr_negative_flag_is_set_for_negative_result() {
    // 1 - 2 => negative result should set only the negative flag among core flags.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_basic_instruction(
        OP_CODE_SUBTRACT_INTEGER,
        0,
        encode_direct_argument(1),
        encode_direct_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let cr = cpu.get_result_register();
    assert!(cr_negative(cr));
    assert!(!cr_zero(cr));
    assert!(!cr_overflow(cr));
}

#[test]
fn cr_overflow_flag_is_set_on_signed_overflow() {
    // i64::MAX + 1 overflows to i64::MIN and must set overflow.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, i64::MAX as u64);
    cpu.set_register(2, 1);
    let instruction = encode_basic_instruction(
        OP_CODE_ADD_INTEGER,
        0,
        encode_register_argument(1),
        encode_register_argument(2),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), i64::MIN as u64);
    let cr = cpu.get_result_register();
    assert!(cr_overflow(cr));
    assert!(cr_negative(cr));
    assert!(!cr_zero(cr));
}

#[test]
fn divi_by_zero_raises_interrupt_without_panicking() {
    // Configure interrupt vector so divide-by-zero jumps to base + interrupt_code(1).
    let (mut cpu, mut memory) = new_cpu_and_memory();
    configure_interrupt_table(&mut cpu, 40, 8);
    let instruction = encode_basic_instruction(
        OP_CODE_DIVIDE_INTEGER,
        0,
        encode_direct_argument(8),
        encode_direct_argument(0),
    );

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_program_counter(), 41);
}
