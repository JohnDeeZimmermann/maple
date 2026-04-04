use crate::common::{execute_single_instruction, new_cpu_and_memory, OP_CODE_POP_PUSH};

fn pop_instruction(rdest: u8) -> u64 {
    create_pop_push_instruction(0, rdest)
}

fn push_instruction(rsrc: u8) -> u64 {
    create_pop_push_instruction(1, rsrc)
}

fn create_pop_push_instruction(options: u8, rdest: u8) -> u64 {
    maple::maple::instructions::instructions::create_basic_instruction(
        maple::maple::instructions::instructions::InstructionArguments {
            op_code: OP_CODE_POP_PUSH,
            options,
            rdest,
            arg1_raw: 0,
            arg2_raw: 0,
        },
    )
}

#[test]
fn pop_reads_value_from_memory_and_increments_sp() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_stack_pointer(100);
    memory.write(100, 0xDEADBEEF, &mut cpu);
    let instruction = pop_instruction(0);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0xDEADBEEF);
    assert_eq!(cpu.get_stack_pointer(), 101);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn push_writes_value_to_memory_and_decrements_sp() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_stack_pointer(100);
    cpu.set_register(0, 0xCAFEBABE);
    let instruction = push_instruction(0);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(100, &mut cpu), 0xCAFEBABE);
    assert_eq!(cpu.get_stack_pointer(), 99);
    assert_eq!(cpu.get_program_counter(), 1);
}

#[test]
fn pop_increments_after_reading() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_stack_pointer(50);
    memory.write(50, 0x1111, &mut cpu);
    memory.write(51, 0x2222, &mut cpu);
    let instruction = pop_instruction(0);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0x1111);
    assert_eq!(cpu.get_stack_pointer(), 51);
}

#[test]
fn pop_stores_to_correct_destination_register() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_stack_pointer(75);
    memory.write(75, 0xAAAA, &mut cpu);
    let instruction = pop_instruction(3);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(3), 0xAAAA);
    assert_eq!(cpu.get_register(0), 0);
}

#[test]
fn push_reads_from_correct_source_register() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_stack_pointer(80);
    cpu.set_register(2, 0xBBBB);
    let instruction = push_instruction(2);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(80, &mut cpu), 0xBBBB);
    assert_eq!(cpu.get_register(2), 0xBBBB);
}

#[test]
fn pop_does_not_modify_memory() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_stack_pointer(60);
    memory.write(60, 0xCCCC, &mut cpu);
    let instruction = pop_instruction(0);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(60, &mut cpu), 0xCCCC);
}

#[test]
fn push_does_not_modify_register() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_stack_pointer(65);
    cpu.set_register(1, 0xDDDD);
    let instruction = push_instruction(1);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(1), 0xDDDD);
    assert_eq!(memory.read(65, &mut cpu), 0xDDDD);
}

#[test]
fn pop_with_zero_stack_pointer_initializes_register() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_stack_pointer(1);
    memory.write(1, 0x1234, &mut cpu);
    let instruction = pop_instruction(0);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0x1234);
    assert_eq!(cpu.get_stack_pointer(), 2);
}

#[test]
fn pop_unknown_option_leaves_sp_unchanged() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_stack_pointer(90);
    memory.write(90, 0xEEEE, &mut cpu);
    let instruction = create_pop_push_instruction(99, 0);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(cpu.get_register(0), 0);
    assert_eq!(cpu.get_stack_pointer(), 90);
}

#[test]
fn push_unknown_option_leaves_sp_unchanged() {
    let (mut cpu, mut memory) = new_cpu_and_memory();
    cpu.set_stack_pointer(95);
    cpu.set_register(0, 0xFFFF);
    let instruction = create_pop_push_instruction(99, 0);

    execute_single_instruction(&mut cpu, &mut memory, instruction);

    assert_eq!(memory.read(95, &mut cpu), 0);
    assert_eq!(cpu.get_stack_pointer(), 95);
}
