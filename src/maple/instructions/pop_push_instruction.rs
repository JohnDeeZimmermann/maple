use crate::maple::cpu::{MapleCPU, CPU};
use crate::maple::instructions::instructions::InstructionArguments;
use crate::maple::memory::Memory;

pub fn execute_pop_push_instruction(
    cpu: &mut MapleCPU,
    memory: &mut Memory,
    args: &InstructionArguments,
) {
    let stack_pointer = cpu.get_stack_pointer() as u32;

    let updated_stack_pointer = match args.options {
        0 => {
            let current_value = memory.read(stack_pointer, cpu);
            cpu.set_register(args.rdest, current_value);
            stack_pointer + 1
        }
        1 => {
            let register_value = cpu.get_register(args.rdest);
            memory.write(stack_pointer, register_value, cpu);
            stack_pointer - 1
        }
        _ => stack_pointer,
    };

    cpu.set_stack_pointer(updated_stack_pointer as u64);
}
