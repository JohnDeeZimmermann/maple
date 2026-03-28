# Compare Float (CMPF)

## Purpose
Performs an IEEE-754 `f64` comparison between two operands and updates the `cr` (condition result) register with the comparison flags. This instruction does not write to any general-purpose register or modify the program counter.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x0C`: `CMPF`

## Parameters and Treatment

1. `op_code` (8 bits)
    - Must be `0x0C` to select compare float.

2. `options` (4 bits)
   - Decoded but currently unused by compare execution.

3. `rdest` (4 bits)
   - **Unused** by this instruction. Set to `0`.

4. `arg1_raw`, `arg2_raw` (24 bits each)
   - Operands to compare.
   - Each is resolved by `resolve_potential_register_argument_value`:
     - Bit 0 = `1`: operand is a register reference; register id comes from bits 4..1.
     - Bit 0 = `0`: operand is a direct literal; value is `arg >> 1`.

## Execution Flow
```text
a = f64::from_bits(resolve(arg1_raw))
b = f64::from_bits(resolve(arg2_raw))
perform_float_compare(cpu, a, b)   // updates cr register only
pc += 1
```

## Condition System
This instruction uses the shared [Condition System](condition_system.md). See that document for details on:
- The `cr` register layout and flag definitions
- How comparison results are encoded in `cr`

## Use Cases
The CMPF instruction is typically paired with conditional instructions that read the `cr` register:

```text
CMPF r1, r2      // compare r1 and r2 as f64, update cr
CSKIP EQ, r3, 0  // skip next if cr indicates equality
```

## Notes
- The comparison is performed as IEEE-754 `f64` subtraction (`a - b`).
- No general-purpose registers are modified.
- The `options` field is reserved for future use; set to `0`.
- Unlike CSKIP, CMPF does not skip or branch; it only updates `cr`.
- Direct arguments are shifted right by 1 bit; ensure proper encoding for `f64` bit patterns.
