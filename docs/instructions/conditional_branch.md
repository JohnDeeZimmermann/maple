# Conditional Branch (CBRANCH)

## Purpose
Conditionally branches to a target address based on the current comparison result stored in the `cr` (condition result) register. The target address is computed by adding an offset to a base address from a register.

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
- `0x0E`: `CBRANCH`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x0E` to select conditional branch.

2. `options` (4 bits)
   - Selects the condition to test against the comparison result.
   - `0`: `EQ` (equal)
   - `1`: `NEQ` (not equal)
   - `2`: `GT` (greater than)
   - `3`: `LT` (less than)
   - `4`: `GTE` (greater than or equal)
   - `5`: `LTE` (less than or equal)

3. `rdest` (4 bits)
   - Specifies the register containing the base address for the branch target.
   - The target address is `rdest_register + offset`.

4. `arg1_raw` (24 bits)
   - The offset to add to the base address.
   - Resolved by `resolve_potential_register_argument_value`:
     - Bit 0 = `1`: offset is a register reference; register id comes from bits 4..1.
     - Bit 0 = `0`: offset is a direct literal; value is `arg1_raw >> 1`.

5. `arg2_raw` (24 bits)
   - **Unused** by this instruction. Set to `0`.

## Execution Flow
```text
result = get_conditional_result(cpu)   // read cr register flags
offset = resolve_potential_register_argument_value(cpu, arg1_raw)
base_address = cpu.get_register(rdest)
target_address = base_address + offset

if is_condition_option_met(options, result):
    safely_update_program_counter(cpu, memory, target_address)
else:
    // no branch, continue to next instruction
```

## Condition System
This instruction uses the shared [Condition System](condition_system.md). See that document for details on:
- The `cr` register layout and flag definitions
- Condition option encodings and evaluation logic

## Notes
- Unlike `CSKIP` which modifies the program counter by skipping instructions, `CBRANCH` performs an absolute jump to a computed address.
- The comparison must be performed by a prior instruction (such as `CMPI` or `CMPF`) that sets the `cr` register.
- The target address computation is performed as unsigned addition of the base address and offset.
- In User mode, `safely_update_program_counter` performs virtual-to-physical address translation before updating the program counter.
- `rdest` is the register containing the base address, not the result destination.