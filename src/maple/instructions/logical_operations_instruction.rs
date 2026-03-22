use crate::maple::cpu::MapleCPU;
use crate::maple::instructions::instructions::InstructionArguments;
use crate::maple::utils::resolve_potential_register_argument_value;

pub fn execute_logical_operations_instruction(cpu: &mut MapleCPU, args: &InstructionArguments) {
    let a = resolve_potential_register_argument_value(cpu, args.arg1_raw as u64);
    let b = resolve_potential_register_argument_value(cpu, args.arg2_raw as u64);

    let result = match args.options {
        0 => a & b,
        1 => a | b,
        2 => a ^ b,
        _ => 0,
    };

    cpu.set_register(args.rdest, result);
}
