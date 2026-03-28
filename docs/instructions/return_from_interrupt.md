# Return from Interrupt (RFI)

## Purpose
Returns from an interrupt handler by restoring the program counter to its value before the interrupt occurred and switching back to User mode.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x19`: `RFI`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x19` to select return from interrupt.

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
address = cpu.get_old_program_counter()
cpu.mode = ExecutionMode::User
cpu.set_program_counter(address)
```

## Details

The `get_old_program_counter()` function retrieves the saved program counter from the `sy` register:
- The last 32 bits of `sy` contain the program counter value at the time of the last interrupt.

## Notes
- This instruction switches the CPU from Kernel mode back to User mode.
- The saved program counter is stored in the `sy` register by the interrupt handling mechanism.
- Typically used at the end of an interrupt handler to resume normal execution.
- This instruction does not modify any general-purpose registers.
- Must be called from Kernel mode (interrupt handlers run in Kernel mode).
