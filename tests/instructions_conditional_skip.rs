mod common;

use common::{
    cr_negative, cr_zero, encode_direct_argument, encode_register_argument,
    execute_single_instruction, new_cpu_and_memory, OP_CODE_CONDITIONAL_SKIP,
};
use maple::maple::cpu::CPU;
use maple::maple::instructions::condition_options::CONDITION_OPTION_EQ;
use maple::maple::instructions::instructions::{create_basic_instruction, InstructionArguments};

#[test]
fn cskip_eq_skips_next_instruction_when_values_match() {
    // arg1 and arg2 are equal, so EQ should skip one extra instruction.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_CONDITIONAL_SKIP,
        options: CONDITION_OPTION_EQ,
        rdest: 0,
        arg1_raw: encode_direct_argument(42),
        arg2_raw: encode_direct_argument(42),
    });

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_program_counter(), 2);
    assert!(cr_zero(cpu.get_result_register()));
    assert!(!cr_negative(cpu.get_result_register()));
}

#[test]
fn cskip_eq_does_not_skip_when_values_differ() {
    // Non-equal values with EQ should continue to the next sequential instruction.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_CONDITIONAL_SKIP,
        options: CONDITION_OPTION_EQ,
        rdest: 0,
        arg1_raw: encode_direct_argument(42),
        arg2_raw: encode_direct_argument(41),
    });

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_program_counter(), 1);
    assert!(!cr_zero(cpu.get_result_register()));
}

#[test]
fn cskip_eq_uses_register_argument_encoding() {
    // arg1 and arg2 resolve from registers when encoded as register references.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(4, 9);
    cpu.set_register(5, 9);
    let instruction = create_basic_instruction(InstructionArguments {
        op_code: OP_CODE_CONDITIONAL_SKIP,
        options: CONDITION_OPTION_EQ,
        rdest: 0,
        arg1_raw: encode_register_argument(4),
        arg2_raw: encode_register_argument(5),
    });

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_program_counter(), 2);
    assert!(cr_zero(cpu.get_result_register()));
}
