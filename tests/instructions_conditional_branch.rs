mod common;

use common::{
    encode_basic_instruction, encode_direct_argument, encode_register_argument,
    execute_single_instruction, new_cpu_and_memory, OP_CODE_COMPARE_INTEGER,
    OP_CODE_CONDITIONAL_BRANCH,
};
use maple::maple::cpu::CPU;
use maple::maple::instructions::condition_options::{
    CONDITION_OPTION_EQ, CONDITION_OPTION_GT, CONDITION_OPTION_GTE, CONDITION_OPTION_LT,
    CONDITION_OPTION_LTE, CONDITION_OPTION_NEQ,
};
use maple::maple::instructions::instructions::{create_basic_instruction, InstructionArguments};

fn cbranch_instruction(options: u8, rdest: u8, arg1_raw: u32) -> u64 {
    create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_CONDITIONAL_BRANCH,
        options,
        rdest,
        arg1_raw,
        arg2_raw: 0,
    })
}

fn do_compare(
    cpu: &mut maple::maple::cpu::MapleCPU,
    memory: &mut maple::maple::memory::Memory,
    a: u64,
    b: u64,
) {
    let cmp = encode_basic_instruction(
        OP_CODE_COMPARE_INTEGER,
        0,
        encode_direct_argument(a as u32),
        encode_direct_argument(b as u32),
    );
    execute_single_instruction(cpu, memory, cmp);
}

#[test]
fn cbranch_takes_branch_when_condition_met() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 42, 42);

    cpu.set_register(1, 100);
    let branch = cbranch_instruction(CONDITION_OPTION_EQ, 1, encode_direct_argument(10));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 110);
}

#[test]
fn cbranch_does_not_branch_when_condition_not_met() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 42, 41);

    cpu.set_register(1, 100);
    let branch = cbranch_instruction(CONDITION_OPTION_EQ, 1, encode_direct_argument(10));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 2);
}

#[test]
fn cbranch_with_neq_branches_when_values_not_equal() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 10, 20);

    cpu.set_register(2, 50);
    let branch = cbranch_instruction(CONDITION_OPTION_NEQ, 2, encode_direct_argument(5));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 55);
}

#[test]
fn cbranch_with_gt_branches_when_greater() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 100, 50);

    cpu.set_register(3, 200);
    let branch = cbranch_instruction(CONDITION_OPTION_GT, 3, encode_direct_argument(25));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 225);
}

#[test]
fn cbranch_with_gt_does_not_branch_when_less() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 50, 100);

    cpu.set_register(3, 200);
    let branch = cbranch_instruction(CONDITION_OPTION_GT, 3, encode_direct_argument(25));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 2);
}

#[test]
fn cbranch_with_lt_branches_when_less() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 10, 50);

    cpu.set_register(4, 300);
    let branch = cbranch_instruction(CONDITION_OPTION_LT, 4, encode_direct_argument(20));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 320);
}

#[test]
fn cbranch_with_gte_branches_when_equal() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 77, 77);

    cpu.set_register(5, 400);
    let branch = cbranch_instruction(CONDITION_OPTION_GTE, 5, encode_direct_argument(15));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 415);
}

#[test]
fn cbranch_with_gte_branches_when_greater() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 100, 50);

    cpu.set_register(5, 400);
    let branch = cbranch_instruction(CONDITION_OPTION_GTE, 5, encode_direct_argument(15));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 415);
}

#[test]
fn cbranch_with_lte_branches_when_equal() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 99, 99);

    cpu.set_register(0, 500);
    let branch = cbranch_instruction(CONDITION_OPTION_LTE, 0, encode_direct_argument(30));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 530);
}

#[test]
fn cbranch_with_lte_branches_when_less() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 50, 100);

    cpu.set_register(0, 500);
    let branch = cbranch_instruction(CONDITION_OPTION_LTE, 0, encode_direct_argument(30));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 530);
}

#[test]
fn cbranch_with_lte_does_not_branch_when_greater() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 100, 50);

    cpu.set_register(0, 500);
    let branch = cbranch_instruction(CONDITION_OPTION_LTE, 0, encode_direct_argument(30));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 2);
}

#[test]
fn cbranch_uses_register_argument_for_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 42, 42);

    cpu.set_register(1, 100);
    cpu.set_register(2, 50);
    let branch = cbranch_instruction(CONDITION_OPTION_EQ, 1, encode_register_argument(2));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 150);
}

#[test]
fn cbranch_zero_offset_stays_at_base() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 5, 5);

    cpu.set_register(2, 1000);
    let branch = cbranch_instruction(CONDITION_OPTION_EQ, 2, encode_direct_argument(0));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 1000);
}

#[test]
fn cbranch_unknown_condition_option_does_not_branch() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    do_compare(&mut cpu, &mut memory, 42, 42);

    cpu.set_register(1, 100);
    let branch = cbranch_instruction(99, 1, encode_direct_argument(10));
    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 2);
}
