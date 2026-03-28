# No Operation (NOP)

## Purpose
Performs no operation. The program counter is incremented by 1 and execution continues with the next instruction.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x00`: `NOP`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x00` to select NOP.

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
pc += 1
```

## Notes
- NOP is commonly used for instruction alignment, timing delays, or as a placeholder for future instructions.
- This instruction does not modify any registers or memory.
- NOP is the only instruction with opcode `0x00`.
