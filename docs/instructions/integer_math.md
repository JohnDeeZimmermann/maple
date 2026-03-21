# Integer Math (ADDI / SUBI / MULI / DIVI)

## Purpose
These instructions perform signed 64-bit integer arithmetic using two decoded operands and write the result to `rdest`.

- `ADDI`: addition
- `SUBI`: subtraction
- `MULI`: multiplication
- `DIVI`: division

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCodes:
- `0x02`: `ADDI`
- `0x03`: `SUBI`
- `0x04`: `MULI`
- `0x05`: `DIVI`

## Parameters and Treatment
1. `op_code` (8 bits)
- Selects which arithmetic operation is executed.

2. `options` (4 bits)
- Decoded but currently unused by integer math execution.

3. `rdest` (4 bits)
- Destination register for the arithmetic result.

4. `arg1_raw`, `arg2_raw` (24 bits each)
- Each operand is resolved by `resolve_potential_register_argument_value`:
  - Bit 0 = `1`: operand is a register reference; register id comes from bits 4..1.
  - Bit 0 = `0`: operand is a direct literal; value is `arg >> 1`.
- Resolved operands are cast to `i64` before arithmetic.

## Execution (Shared Flow)
```text
a = resolve(arg1_raw) as i64
b = resolve(arg2_raw) as i64
(result, overflowed) = a <op> b   // overflowing_* variant
rdest = result as u64
update_cr(result, overflowed)
```

For `DIVI` only:
- If `b == 0`, an `INVALID_DIVISION_BY_ZERO` interrupt is raised and execution returns early.
- In that case, no result is written and `cr` is not updated by this instruction path.

## Conditional Result Register (`cr`) Update
After successful arithmetic, the low 4 bits of `cr` are updated and higher bits are preserved.

Bit mapping:
- bit 0: `overflow` (`overflowed` from Rust `overflowing_*`)
- bit 1: `zero` (`result == 0`)
- bit 2: `negative` (MSB of `result`)
- bit 3: `parity` (1 when `result` has an even number of set bits)

Equivalent write:
```text
cr = (cr & 0xFFFFFFFFFFFFFFF0) | parity<<3 | negative<<2 | zero<<1 | overflow
```

## Notes
- Arithmetic is signed (`i64`) and wraps on overflow using Rust `overflowing_*` behavior.
- Final register storage is raw 64-bit (`u64`) bit pattern of the signed result.
