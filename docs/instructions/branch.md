# Branch (BR)

## Purpose
Unconditionally branches to a target address computed by adding an offset to a base address from a register. The target address is `base_address + offset`.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x0F`: `BR`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x0F` to select branch.

2. `options` (4 bits)
   - **Unused** by this instruction. Set to `0`.

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
offset = resolve_potential_register_argument_value(cpu, arg1_raw)
base_address = cpu.get_register(rdest)
target_address = base_address + offset

safely_update_program_counter(cpu, memory, target_address)
```

## Notes
- Unlike `CBRANCH` which conditionally branches based on the condition register, `BR` always branches unconditionally.
- The target address computation is performed as unsigned addition of the base address and offset.
- In User mode, `safely_update_program_counter` performs virtual-to-physical address translation before updating the program counter.
- `rdest` is the register containing the base address, not the result destination.
