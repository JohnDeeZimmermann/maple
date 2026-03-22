# Exit (EXIT)

## Purpose
Powers off the processor by returning ExecutionResult::Exit. This instruction is only allowed in Kernel mode. In User mode, it raises an illegal exit interrupt.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x16`: `EXIT`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x16` to select exit.

2. `options` (4 bits)
   - **Unused** by this instruction. Set to `0`.

3. `rdest` (4 bits)
   - **Unused** by this instruction. Set to `0`.

4. `arg1_raw` (24 bits)
   - **Unused** by this instruction. Set to `0`.

5. `arg2_raw` (24 bits)
   - **Unused** by this instruction. Set to `0`.

## Execution Flow
```text
if cpu.mode == ExecutionMode::User:
    cpu.raise_interrupt(INTERRUPT_CODE_ILLEGAL_EXIT)
    return ExecutionResult::Ok
else:
    return ExecutionResult::Exit
```

## Notes
- In Kernel mode, this instruction terminates the virtual machine execution.
- In User mode, this instruction raises interrupt code 7 (ILLEGAL_EXIT) instead of exiting.
- Exit is not possible in User mode as a safety mechanism.
- This instruction does not modify any registers.
