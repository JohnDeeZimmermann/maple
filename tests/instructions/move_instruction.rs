use crate::common::{encode_move_instruction, execute_single_instruction, new_cpu_and_memory};

#[test]
fn mov_direct_value_writes_destination_register() {
    // MOV with a direct source copies the literal into rdest.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_move_instruction(false, 2, false, 0x1234, false);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(2), 0x1234);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn mov_register_source_resolves_register_value() {
    // MOV with register source reads the source register and writes it to rdest.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(4, 0xDEAD_BEEF);
    let instruction = encode_move_instruction(false, 1, false, 4, true);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(1), 0xDEAD_BEEF);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn mov_sign_bit_sets_result_msb() {
    // The dedicated sign bit should be OR-ed into the destination MSB.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_move_instruction(false, 3, true, 0x45, false);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(3), (1_u64 << 63) | 0x45);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn mvn_direct_value_writes_inverted_result() {
    // MVN inverts the computed source value before storing it.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    let instruction = encode_move_instruction(true, 0, false, 0x0F, false);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), !0x0F_u64);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn mvn_register_source_writes_inverted_register_value() {
    // MVN + register source still applies the sign bit before the final NOT.
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_register(5, 0x55);
    let instruction = encode_move_instruction(true, 2, true, 5, true);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    let expected_value = !((1_u64 << 63) | 0x55);
    assert_eq!(cpu.get_register(2), expected_value);
    assert_eq!(cpu.get_program_counter(), 1);
}
