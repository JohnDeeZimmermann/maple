mod common;

use common::{
    cr_negative, cr_zero, encode_basic_instruction, encode_direct_argument,
    execute_single_instruction, new_cpu_and_memory, OP_CODE_COMPARE_INTEGER,
    OP_CODE_COMPARE_RESULTS,
};

use maple::maple::instructions::condition_options::{
    CONDITION_OPTION_EQ, CONDITION_OPTION_GT, CONDITION_OPTION_GTE, CONDITION_OPTION_LT,
    CONDITION_OPTION_LTE, CONDITION_OPTION_NEQ,
};
use maple::maple::instructions::instructions::{create_basic_instruction, InstructionArguments};

#[test]
fn compare_results_eq_returns_true_when_equal() {
    // After CMPI sets zero flag, REQ should return 1.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(42),
        encode_direct_argument(42),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    let rge_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: CONDITION_OPTION_EQ,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, rge_instruction);

    assert_eq!(cpu.get_register(1), 1);
}

#[test]
fn compare_results_eq_returns_false_when_not_equal() {
    // After CMPI with unequal values, REQ should return 0.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(42),
        encode_direct_argument(41),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    let req_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: CONDITION_OPTION_EQ,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, req_instruction);

    assert_eq!(cpu.get_register(1), 0);
}

#[test]
fn compare_results_neq_returns_true_when_not_equal() {
    // After CMPI with unequal values, RNQ should return 1.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(42),
        encode_direct_argument(41),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    let rnq_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: CONDITION_OPTION_NEQ,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, rnq_instruction);

    assert_eq!(cpu.get_register(1), 1);
}

#[test]
fn compare_results_gt_returns_true_when_greater() {
    // After CMPI where a > b, RGT should return 1.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(10),
        encode_direct_argument(5),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    assert!(!cr_zero(cpu.get_result_register()));
    assert!(!cr_negative(cpu.get_result_register()));

    let rgt_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: CONDITION_OPTION_GT,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, rgt_instruction);

    assert_eq!(cpu.get_register(1), 1);
}

#[test]
fn compare_results_gt_returns_false_when_less() {
    // After CMPI where a < b, RGT should return 0.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(5),
        encode_direct_argument(10),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    assert!(cr_negative(cpu.get_result_register()));

    let rgt_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: CONDITION_OPTION_GT,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, rgt_instruction);

    assert_eq!(cpu.get_register(1), 0);
}

#[test]
fn compare_results_lt_returns_true_when_less() {
    // After CMPI where a < b, RLT should return 1.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(5),
        encode_direct_argument(10),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    let rlt_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: CONDITION_OPTION_LT,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, rlt_instruction);

    assert_eq!(cpu.get_register(1), 1);
}

#[test]
fn compare_results_gte_returns_true_when_equal() {
    // After CMPI where a == b, RGE should return 1.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(42),
        encode_direct_argument(42),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    let rge_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: CONDITION_OPTION_GTE,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, rge_instruction);

    assert_eq!(cpu.get_register(1), 1);
}

#[test]
fn compare_results_gte_returns_true_when_greater() {
    // After CMPI where a > b, RGE should return 1.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(10),
        encode_direct_argument(5),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    let rge_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: CONDITION_OPTION_GTE,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, rge_instruction);

    assert_eq!(cpu.get_register(1), 1);
}

#[test]
fn compare_results_lte_returns_true_when_equal() {
    // After CMPI where a == b, RLE should return 1.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(42),
        encode_direct_argument(42),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    let rle_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: CONDITION_OPTION_LTE,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, rle_instruction);

    assert_eq!(cpu.get_register(1), 1);
}

#[test]
fn compare_results_lte_returns_true_when_less() {
    // After CMPI where a < b, RLE should return 1.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(5),
        encode_direct_argument(10),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    let rle_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: CONDITION_OPTION_LTE,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, rle_instruction);

    assert_eq!(cpu.get_register(1), 1);
}

#[test]
fn compare_results_lte_returns_false_when_greater() {
    // After CMPI where a > b, RLE should return 0.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(10),
        encode_direct_argument(5),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    let rle_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: CONDITION_OPTION_LTE,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, rle_instruction);

    assert_eq!(cpu.get_register(1), 0);
}

#[test]
fn compare_results_returns_zero_for_unknown_option() {
    // Unknown option should return 0.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let cmp_instruction = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(42),
        encode_direct_argument(42),
    );
    execute_single_instruction(&mut cpu, &mut memory, cmp_instruction);

    let unknown_instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_COMPARE_RESULTS,
        options: 99,
        rdest: 1,
        arg1_raw: 0,
        arg2_raw: 0,
    });
    execute_single_instruction(&mut cpu, &mut memory, unknown_instruction);

    assert_eq!(cpu.get_register(1), 0);
}
