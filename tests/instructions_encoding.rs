use maple::maple::instructions::instructions::{create_basic_instruction, InstructionArguments};

#[test]
fn create_basic_instruction_packs_all_fields_into_correct_bit_positions() {
    let args = InstructionArguments {
        op_code: 0xAB,
        options: 0xC,
        rdest: 0x5,
        arg1_raw: 0x65_4321,
        arg2_raw: 0x12_34AB,
    };

    let instruction = create_basic_instruction(args);
    let expected = (0xAB_u64 << 56)
        | (0xC_u64 << 52)
        | (0x5_u64 << 48)
        | (0x65_4321_u64 << 24)
        | 0x12_34AB_u64;

    assert_eq!(instruction, expected);
}

#[test]
fn create_basic_instruction_truncates_inputs_to_field_widths() {
    let args = InstructionArguments {
        op_code: 0xFF,
        options: 0xFF,
        rdest: 0xFF,
        arg1_raw: 0xFFFF_FFFF,
        arg2_raw: 0xABCDEF12,
    };

    let instruction = create_basic_instruction(args);
    let expected = (0xFF_u64 << 56)
        | (0xF_u64 << 52)
        | (0xF_u64 << 48)
        | (0xFF_FFFF_u64 << 24)
        | 0xCDEF12_u64;

    assert_eq!(instruction, expected);
}
