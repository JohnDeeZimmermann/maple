# Float Math (ADDF / SUBF / MULF / DIVF)

## Purpose
These instructions perform IEEE-754 `f64` arithmetic on two source registers and write the `f64` result bits to `rdest`.

- `ADDF`: addition
- `SUBF`: subtraction
- `MULF`: multiplication
- `DIVF`: division

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

(Opcode mapping for these instructions is defined outside this file.)

## Parameters and Treatment
1. `op_code` (8 bits)
- Selects float operation in the instruction dispatcher.

2. `options` (4 bits)
- Decoded but not used by float math execution.

3. `rdest` (4 bits)
- Destination register for the resulting `f64` bit pattern.

4. `arg1_raw`, `arg2_raw` (24 bits each)
- Resolved by `resolve_required_register_argument_value` (register-only):
  - Bit 0 must be `1` (register argument).
  - Register id is read from bits 4..1.
- If bit 0 is `0` (direct argument), `ILLEGAL_DIRECT_ARGUMENT` interrupt is raised and the resolved value becomes `0`.

## Execution (Shared Flow)
```text
a_bits = resolve_required_register(arg1_raw)
b_bits = resolve_required_register(arg2_raw)
a = f64::from_bits(a_bits)
b = f64::from_bits(b_bits)
result = a <op> b
rdest = result.to_bits()
```

## DIVF Special Case
For `DIVF`:
- If `b == 0.0`, `INVALID_DIVISION_BY_ZERO` interrupt is raised.
- Execution does not return early; division is still performed and `rdest` is still written.

## Notes
- No `cr` flags are updated by these float instruction functions.
- Register values are interpreted and stored as raw `f64` bit patterns (`from_bits` / `to_bits`).
