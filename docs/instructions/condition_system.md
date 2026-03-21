# Condition System

## Overview
The Maple CPU provides a shared condition evaluation system used by conditional instructions. This system uses the `cr` (condition result) register to store comparison flags and provides condition options to test against those flags.

## Conditional Result Register (`cr`)
The low 4 bits of `cr` store flags from the most recent comparison. The high 60 bits are preserved during updates.

### Bit Layout
```
| 63..4 | 3 | 2 | 1 | 0 |
|-------|---|---|---|---|
| preserved | parity | negative | zero | overflow |
```

### Flag Definitions
- **bit 0 (overflow)**: Set when the comparison subtraction overflows (signed overflow).
- **bit 1 (zero)**: Set when `a - b == 0` (i.e., `a == b`).
- **bit 2 (negative)**: Set when the MSB of `a - b` is 1 (i.e., `a < b` for signed comparison).
- **bit 3 (parity)**: Set when `a - b` has an even number of set bits.

### Update Formula
```text
cr = (cr & 0xFFFFFFFFFFFFFFF0) | (parity << 3) | (negative << 2) | (zero << 1) | overflow
```

## Condition Options
When executing a conditional instruction, the 4-bit `options` field selects which condition to test against the `cr` flags.

### Encoding
| Value | Name | Description | Condition |
|-------|------|-------------|-----------|
| 0 | `EQ` | Equal | `a == b` |
| 1 | `NEQ` | Not Equal | `a != b` |
| 2 | `GT` | Greater Than | `a > b` |
| 3 | `LT` | Less Than | `a < b` |
| 4 | `GTE` | Greater Than or Equal | `a >= b` |
| 5 | `LTE` | Less Than or Equal | `a <= b` |

### Evaluation Logic
Conditions are evaluated from the `zero` and `negative` flags:
- `EQ`: `result.zero == 1`
- `NEQ`: `result.zero == 0`
- `GT`: `result.zero == 0 && result.negative == 0`
- `LT`: `result.zero == 0 && result.negative == 1`
- `GTE`: `result.zero == 1 || result.negative == 0`
- `LTE`: `result.zero == 1 || result.negative == 1`

## Instructions Using This System
- **CMP** (Compare): Performs comparison and updates `cr`, but does not branch.
- **CSKIP** (Conditional Skip): Performs comparison and skips the next instruction if the condition is met.

## Notes
- Comparisons are always signed 64-bit integer comparisons.
- The `overflow` flag is computed but not used for condition evaluation.
- Multiple conditional instructions can share the same `cr` state (e.g., CMP followed by CSKIP).
