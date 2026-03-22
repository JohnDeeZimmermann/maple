# Branch and Link (BL)

## Purpose
Unconditionally branches to a target address computed by adding an offset to a base address from a register, and saves the address of the following instruction in the `dl` (dynamic link) register. The target address is `base_address + offset`.

This instruction is typically used for subroutine calls, where `dl` serves as the return address.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x10`: `BL`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x10` to select branch and link.

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
link = cpu.get_program_counter() + 1

if safely_update_program_counter(cpu, memory, target_address):
    cpu.set_dynamic_link(link)
```

## Notes
- Unlike `BR` which only performs a jump, `BL` additionally saves the return address in the `dl` register.
- The `dl` register contains the address of the instruction immediately following the `BL` instruction, making it suitable for return navigation.
- The target address computation is performed as unsigned addition of the base address and offset.
- In User mode, `safely_update_program_counter` performs virtual-to-physical address translation before updating the program counter.
- `rdest` is the register containing the base address, not the result destination.
- If the branch fails (e.g., due to an invalid address in User mode), the `dl` register is not updated.
