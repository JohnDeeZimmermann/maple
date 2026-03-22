# Software Interrupt (SWI)

## Purpose
Causes a software interrupt with the given code. The code defines the offset in the software interrupt table defined in the system info register (`sy`). The processor switches to Kernel mode and jumps to the interrupt handler address.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x18`: `SWI`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x18` to select software interrupt.

2. `options` (4 bits)
   - **Unused** by this instruction. Set to `0`.

3. `rdest` (4 bits)
   - **Unused** by this instruction. Set to `0`.

4. `arg1_raw` (24 bits)
   - Contains the interrupt code. The low bit determines if this is a register (`1`) or direct value (`0`).
   - If direct: `(value << 1)`, resolved as `value` by right-shifting 1.
   - If register: `((reg_num << 1) | 1)`, resolved by reading the register value.

5. `arg2_raw` (24 bits)
   - **Unused** by this instruction. Set to `0`.

## Execution Flow
```text
interrupt_code = resolve_potential_register_argument_value(cpu, args.arg1_raw)
cpu.raise_interrupt(interrupt_code as u16)
```

The `raise_interrupt` function:
1. Extracts `interrupt_table_base` from `sy[0..15]`
2. Extracts `interrupt_table_size` from `sy[16..31]`
3. If `interrupt_code > interrupt_table_size`, raises `INTERRUPT_CODE_INVALID_INTERRUPT_CODE`
4. Sets `cpu.mode = ExecutionMode::Kernel`
5. Stores current `program_counter` in `sy[32..63]` (old PC)
6. Sets `program_counter = interrupt_table_base + interrupt_code`

## Examples

### Direct interrupt code
```
SWI 5
```
Encoded:
```
00011000 0000 0000 000000000000000000001010 0 000000000000000000000000 0
OPCODE   OPT DEST ARG1 (code=5, direct)      R ARG2                       R
```

### Register-based interrupt code
```
SWI r2
```
Where r2 contains the interrupt code.

## Notes
- The interrupt table base and size are stored in the system info register (`sy`).
- The old program counter is preserved in the lower 32 bits of `sy` for returning from the interrupt.
- This instruction does not modify any general-purpose registers.
- The processor always switches to Kernel mode when handling a software interrupt.
