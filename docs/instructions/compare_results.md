# Compare Results (RGE/RLE/REQ/RNQ/RGT/RLT)

## Purpose
Evaluates the comparison stored in the `cr` register and writes `true` (1) or `false` (0) to a destination register. This instruction reads the `cr` flags set by a previous comparison (CMPI or CMPF) and produces a boolean result based on the specified condition.

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
- `0x0D`: Compare Results

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x0D` to select Compare Results.

2. `options` (4 bits)
   - Selects the condition to evaluate against the `cr` flags.
   - `0`: `EQ` (equal)
   - `1`: `NEQ` (not equal)
   - `2`: `GT` (greater than)
   - `3`: `LT` (less than)
   - `4`: `GTE` (greater than or equal)
   - `5`: `LTE` (less than or equal)

3. `rdest` (4 bits)
   - Destination register where the boolean result is written.
   - `1` if condition is met, `0` otherwise.

4. `arg1_raw`, `arg2_raw` (24 bits each)
   - **Unused** by this instruction. Set to `0`.

## Execution Flow
```text
result = get_conditional_result(cpu)  // reads flags from cr
condition_met = is_condition_option_met(options, result)
rdest = condition_met ? 1 : 0
pc += 1
```

## Condition Evaluation
Conditions are evaluated from the `zero` and `negative` flags in `cr`:

| Option | Condition | Evaluation |
|--------|-----------|------------|
| `EQ` | a == b | `result.zero == 1` |
| `NEQ` | a != b | `result.zero == 0` |
| `GT` | a > b | `!result.zero && !result.negative` |
| `LT` | a < b | `!result.zero && result.negative` |
| `GTE` | a >= b | `result.zero \|\| !result.negative` |
| `LTE` | a <= b | `result.zero \|\| result.negative` |

## Condition System
This instruction uses the shared [Condition System](condition_system.md). See that document for details on:
- The `cr` register layout and flag definitions
- How comparison instructions set the flags

## Notes
- Must be preceded by a comparison instruction (CMPI or CMPF) to set the `cr` flags.
- `arg1_raw` and `arg2_raw` are ignored; only `options` and `rdest` matter.
- If an unknown option is provided, the result is `0`.