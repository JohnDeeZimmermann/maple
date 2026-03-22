# Logical Shift Left / Logical Shift Right (LSL / LSR)

## Purpose
Performs a logical shift operation on a value. LSL shifts bits left (toward higher-order bits), LSR shifts bits right (toward lower-order bits). The result is stored in `rdest`.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x11`: `LSL/LSR`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x11` to select logical shift.

2. `options` (4 bits)
   - `0`: Logical Shift Left (LSL)
   - `1`: Logical Shift Right (LSR)
   - Other: Returns 0

3. `rdest` (4 bits)
   - Specifies the destination register for the result.

4. `arg1_raw` (24 bits)
   - The value to be shifted.
   - Resolved by `resolve_potential_register_argument_value`:
     - Bit 0 = `1`: value is a register reference; register id comes from bits 4..1.
     - Bit 0 = `0`: value is a direct literal; value is `arg1_raw >> 1`.

5. `arg2_raw` (24 bits)
   - The shift amount.
   - Resolved by `resolve_potential_register_argument_value`:
     - Bit 0 = `1`: shift amount is a register reference.
     - Bit 0 = `0`: shift amount is a direct literal.

## Execution Flow
```text
a = resolve_potential_register_argument_value(cpu, arg1_raw)
b = resolve_potential_register_argument_value(cpu, arg2_raw)

result = if b >= 64:
    0
else:
    match options:
        0 => a.wrapping_shl(b)  // LSL
        1 => a.wrapping_shr(b)  // LSR
        _ => 0

cpu.set_register(rdest, result)
```

## Notes
- LSL fills the lower bits with zeros as bits are shifted left.
- LSR fills the upper bits (bits 64-bits_shift) with zeros as bits are shifted right.
- A shift amount of 0 returns the original value unchanged.
- Shifting by 64 or more bits uses wrapping shift - the result is 0.
