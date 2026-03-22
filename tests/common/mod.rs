use maple::maple::cpu::{ExecutionMode, ExecutionResult, MapleCPU, CPU};
use maple::maple::instructions::instructions::{create_basic_instruction, InstructionArguments};
use maple::maple::memory::Memory;

pub const OP_CODE_MOVE: u8 = 1;
pub const OP_CODE_ADD_INTEGER: u8 = 2;
pub const OP_CODE_SUBTRACT_INTEGER: u8 = 3;
pub const OP_CODE_MULTIPLY_INTEGER: u8 = 4;
pub const OP_CODE_DIVIDE_INTEGER: u8 = 5;
pub const OP_CODE_ADD_FLOAT: u8 = 6;
pub const OP_CODE_SUBTRACT_FLOAT: u8 = 7;
pub const OP_CODE_MULTIPLY_FLOAT: u8 = 8;
pub const OP_CODE_DIVIDE_FLOAT: u8 = 9;
pub const OP_CODE_CONDITIONAL_SKIP: u8 = 10;
pub const OP_CODE_COMPARE_INTEGER: u8 = 11;
pub const OP_CODE_COMPARE_FLOAT: u8 = 12;
pub const OP_CODE_COMPARE_RESULTS: u8 = 13;

pub fn new_cpu_and_memory() -> (MapleCPU, Memory) {
    // All instruction tests run in kernel mode with PC starting at 0.
    let mut cpu = MapleCPU::new();
    cpu.mode = ExecutionMode::Kernel;
    cpu.set_program_counter(0);

    let memory = Memory::new(256);

    (cpu, memory)
}

pub fn execute_single_instruction(
    cpu: &mut MapleCPU,
    memory: &mut Memory,
    instruction: u64,
) -> ExecutionResult {
    // Mirror the test contract: write one instruction at current PC and step once.
    let pc = cpu.get_program_counter() as u32;
    memory.write(pc, instruction, cpu);
    cpu.process(memory)
}

pub fn encode_direct_argument(value: u32) -> u32 {
    // Direct arguments use the low bit as a "register flag" (0 for direct).
    (value & 0x7F_FFFF) << 1
}

pub fn encode_register_argument(register: u8) -> u32 {
    // Register arguments set low bit to 1 and store register id in the next bits.
    (((register as u32) & 0xF) << 1) | 1
}

pub fn encode_basic_instruction(op_code: u8, rdest: u8, arg1_raw: u32, arg2_raw: u32) -> u64 {
    create_basic_instruction(InstructionArguments {
        op_code,
        options: 0,
        rdest,
        arg1_raw,
        arg2_raw,
    })
}

pub fn encode_move_instruction(
    is_move_not: bool,
    rdest: u8,
    sign: bool,
    source_value: u64,
    source_is_register: bool,
) -> u64 {
    // MOV/MVN have their own binary layout and are not built via create_basic_instruction.
    let raw_source = if source_is_register {
        ((source_value & 0xF) << 1) | 1
    } else {
        (source_value & 0x1_FFFF_FFFF_FFFF) << 1
    };

    ((OP_CODE_MOVE as u64) << 56)
        | ((is_move_not as u64) << 55)
        | (((rdest as u64) & 0xF) << 51)
        | ((sign as u64) << 50)
        | (raw_source & 0x3_FFFF_FFFF_FFFF)
}

pub fn configure_interrupt_table(cpu: &mut MapleCPU, base: u16, size: u16) {
    // sy[0..15]=base and sy[16..31]=size, both from the MSB side.
    let system_info = ((base as u64) << 48) | ((size as u64) << 32);
    cpu.set_system_info(system_info);
}

pub fn cr_overflow(cr: u64) -> bool {
    (cr & 0b0001) != 0
}

pub fn cr_zero(cr: u64) -> bool {
    (cr & 0b0010) != 0
}

pub fn cr_negative(cr: u64) -> bool {
    (cr & 0b0100) != 0
}

pub fn cr_parity(cr: u64) -> bool {
    (cr & 0b1000) != 0
}
