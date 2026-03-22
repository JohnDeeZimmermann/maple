# Pop / Push (POP / PUSH)

## Purpose
Pop reads a value from memory at the stack pointer address and increments the stack pointer. Push decrements the stack pointer and writes a register value to memory at the new stack pointer address.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x15`: `POP/PUSH`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x15` to select pop/push.

2. `options` (4 bits)
   - `0`: POP - reads from memory and increments stack pointer
   - `1`: PUSH - decrements stack pointer and writes to memory
   - Other: No operation, stack pointer unchanged

3. `rdest` (4 bits)
   - For POP: specifies the destination register for the loaded value
   - For PUSH: specifies the source register whose value will be stored

4. `arg1_raw` (24 bits)
   - **Unused** by this instruction. Set to `0`.

5. `arg2_raw` (24 bits)
   - **Unused** by this instruction. Set to `0`.

## Execution Flow
```text
stack_pointer = cpu.get_stack_pointer()

match options:
    0:  // POP
        value = memory.read(stack_pointer, cpu)
        cpu.set_register(rdest, value)
        updated_stack_pointer = stack_pointer + 1

    1:  // PUSH
        value = cpu.get_register(rdest)
        memory.write(stack_pointer, value, cpu)
        updated_stack_pointer = stack_pointer - 1

    _:
        updated_stack_pointer = stack_pointer

cpu.set_stack_pointer(updated_stack_pointer)
```

## Notes
- POP reads from the current stack pointer location and then increments the stack pointer.
- PUSH writes to the current stack pointer location and then decrements the stack pointer.
- The stack grows downward (toward lower addresses) as values are pushed.
- In User mode, memory access uses virtual address translation.
- This instruction does not modify the program counter; it is incremented automatically after execution.
