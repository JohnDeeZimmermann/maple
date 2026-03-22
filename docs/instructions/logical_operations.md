# Logical Operations (AND / ORR / XOR)

## Purpose
Performs bitwise logical operations on two operands. The result is stored in `rdest`.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x12`: `AND/ORR/XOR`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x12` to select logical operations.

2. `options` (4 bits)
   - `0`: AND - bitwise AND of both operands
   - `1`: ORR - bitwise OR of both operands
   - `2`: XOR - bitwise XOR of both operands
   - Other: Returns 0

3. `rdest` (4 bits)
   - Specifies the destination register for the result.

4. `arg1_raw` (24 bits)
   - The first operand.
   - Resolved by `resolve_potential_register_argument_value`:
     - Bit 0 = `1`: operand is a register reference; register id comes from bits 4..1.
     - Bit 0 = `0`: operand is a direct literal; value is `arg1_raw >> 1`.

5. `arg2_raw` (24 bits)
   - The second operand.
   - Resolved by `resolve_potential_register_argument_value`:
     - Bit 0 = `1`: operand is a register reference.
     - Bit 0 = `0`: operand is a direct literal.

## Execution Flow
```text
a = resolve_potential_register_argument_value(cpu, arg1_raw)
b = resolve_potential_register_argument_value(cpu, arg2_raw)

result = match options:
    0 => a & b  // AND
    1 => a | b  // ORR
    2 => a ^ b  // XOR
    _ => 0

cpu.set_register(rdest, result)
```

## Notes
- AND: Result has 1s only where both operands have 1s.
- ORR: Result has 1s where either operand has 1s.
- XOR: Result has 1s where exactly one operand has 1 (not both).
- AND with all 1s returns the other operand unchanged.
- XOR with all 1s returns the bitwise NOT of the other operand.
- XOR of a value with itself returns 0.
