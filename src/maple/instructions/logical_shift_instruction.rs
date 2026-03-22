use crate::maple::cpu::{MapleCPU, CPU};
use crate::maple::instructions::instructions::InstructionArguments;
use crate::maple::utils::resolve_potential_register_argument_value;

pub fn execute_logical_shift_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let a = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64);
    let b = resolve_potential_register_argument_value(cpu, args.arg2_raw as u64);

    let result = if b >= 64 {
        0
    } else {
        match args.options {
            0 => a.wrapping_shl(b as u32),
            1 => a.wrapping_shr(b as u32),
            _ => 0,
        }
    };

    cpu.set_register(args.rdest, result);
}
