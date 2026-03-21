# MOV / MVN

## Purpose
`MOV` copies a decoded source value into `rdest`.
`MVN` performs the same decode, then stores the bitwise NOT of that value.

## Binary Layout
```
OPCODE (8) | OPT (1) | RDEST (4) | SIGN (1) | VALUE (50)
```
- `OPCODE`: must be `0x01` (move instruction family).
- `OPT`: selects variant.
  - `0`: `MOV`
  - `1`: `MVN`
- `RDEST`: 4-bit destination register id.
- `SIGN`: forced MSB bit for the intermediate value.
- `VALUE`: packed source argument (payload + register/direct flag).

## Parameters and Treatment
1. `opt` (`OPT`, 1 bit)
- Decoded from bit 55.
- `opt == 1` means `MVN`, otherwise `MOV`.

2. `rdest` (`RDEST`, 4 bits)
- Decoded from bits 54..51.
- Written unconditionally with the final result.

3. `sign` (`SIGN`, 1 bit)
- Decoded from bit 50.
- Applied as `value_with_sign = (sign << 63) | actual_value`.
- This always affects the intermediate value before any `MVN` inversion.

4. `raw_value` (`VALUE`, 50 bits)
- Decoded from bits 49..0.
- Interpreted by `resolve_potential_register_argument_value`:
  - If bit 0 is `1`: treat as register argument, read register id from bits 4..1.
  - If bit 0 is `0`: treat as direct argument, use `raw_value >> 1` as the literal.

## Execution Order
```text
actual_value    = decode(raw_value)
value_with_sign = (sign << 63) | actual_value
result          = (opt == 1) ? ~value_with_sign : value_with_sign
RDEST           = result
```

## Notes
- `SIGN` is not arithmetic sign extension; it is a direct OR into bit 63.
- For `MVN`, inversion happens after `SIGN` has been applied.
