# Conditional Skip (CSKIP)

## Purpose
Compares two decoded operands and conditionally skips the next instruction based on the comparison result. The `cr` (condition result) register is updated with flags from the comparison.

Supported conditions:
- `EQ`: equal (a == b)
- `NEQ`: not equal (a != b)
- `GT`: greater than (a > b)
- `LT`: less than (a < b)
- `GTE`: greater than or equal (a >= b)
- `LTE`: less than or equal (a <= b)

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x06`: `CSKIP`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x06` to select conditional skip.

2. `options` (4 bits)
   - Selects the condition to test against the comparison result.
   - `0`: `EQ` (equal)
   - `1`: `NEQ` (not equal)
   - `2`: `GT` (greater than)
   - `3`: `LT` (less than)
   - `4`: `GTE` (greater than or equal)
   - `5`: `LTE` (less than or equal)

3. `rdest` (4 bits)
   - **Unused** by this instruction. Set to `0`.

4. `arg1_raw`, `arg2_raw` (24 bits each)
   - Operands to compare.
   - Each is resolved by `resolve_potential_register_argument_value`:
     - Bit 0 = `1`: operand is a register reference; register id comes from bits 4..1.
     - Bit 0 = `0`: operand is a direct literal; value is `arg >> 1`.

## Execution Flow
```text
a = resolve(arg1_raw) as i64
b = resolve(arg2_raw) as i64
perform_int_compare(cpu, a, b)   // updates cr register
result = get_conditional_result(cpu)
if is_condition_option_met(options, result):
    pc += 2   // skip next instruction
else:
    pc += 1   // continue normally
```

## Condition System
This instruction uses the shared [Condition System](condition_system.md). See that document for details on:
- The `cr` register layout and flag definitions
- Condition option encodings and evaluation logic

## Notes
- The comparison is performed as signed 64-bit integer subtraction (`a - b`).
- The subtraction itself is performed with `overflowing_sub`, but `overflow` flag is not used for condition evaluation.
- When the condition is met, the program counter is incremented twice, effectively skipping the next instruction.
- `rdest` is ignored entirely; use `0` for clarity.
