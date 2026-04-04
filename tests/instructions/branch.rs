use crate::common::{
    encode_direct_argument, encode_register_argument, execute_single_instruction,
    new_cpu_and_memory, OP_CODE_BRANCH,
};
use maple::maple::instructions::instructions::{create_basic_instruction, InstructionArguments};

fn branch_instruction(rdest: u8, arg1_raw: u32) -> u64 {
    create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_BRANCH,
        options: 0,
        rdest,
        arg1_raw,
        arg2_raw: 0,
    })
}

#[test]
fn branch_with_direct_offset_jumps_to_target() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 100);
    let branch = branch_instruction(1, encode_direct_argument(50));

    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 150);
}

#[test]
fn branch_with_register_offset_uses_register_value() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 200);
    cpu.set_register(2, 75);
    let branch = branch_instruction(1, encode_register_argument(2));

    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 275);
}

#[test]
fn branch_with_zero_offset_stays_at_base() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(3, 500);
    let branch = branch_instruction(3, encode_direct_argument(0));

    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 500);
}

#[test]
fn branch_always_takes_branch_unconditionally() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(0, 1000);
    let branch = branch_instruction(0, encode_direct_argument(25));

    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 1025);
}

#[test]
fn branch_combines_base_and_offset_with_addition() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(4, 100);
    cpu.set_register(5, 200);
    let branch = branch_instruction(4, encode_register_argument(5));

    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 300);
}

#[test]
fn branch_with_large_offset() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(1, 0);
    let branch = branch_instruction(1, encode_direct_argument(10000));

    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 10000);
}

#[test]
fn branch_sets_program_counter_not_register() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(2, 77);
    let branch = branch_instruction(2, encode_direct_argument(33));

    execute_single_instruction(&mut cpu, &mut memory, branch);

    assert_eq!(cpu.get_program_counter(), 110);
    assert_eq!(cpu.get_register(2), 77);
}
