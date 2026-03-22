# Store to Memory from Register (STR)

## Purpose
Stores a value from a register into memory. The memory address is computed by adding an offset to a base address from a register.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x14`: `STR`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x14` to select store to memory.

2. `options` (4 bits)
   - **Unused** by this instruction. Set to `0`.

3. `rdest` (4 bits)
   - Specifies the source register whose value will be stored to memory.

4. `arg1_raw` (24 bits)
   - The base address register.
   - Resolved by `resolve_potential_register_argument_value`:
     - Bit 0 = `1`: base address is a register reference; register id comes from bits 4..1.
     - Bit 0 = `0`: base address is a direct literal; value is `arg1_raw >> 1`.

5. `arg2_raw` (24 bits)
   - The offset to add to the base address.
   - Resolved by `resolve_potential_register_argument_value`:
     - Bit 0 = `1`: offset is a register reference.
     - Bit 0 = `0`: offset is a direct literal.

## Execution Flow
```text
base = resolve_potential_register_argument_value(cpu, arg1_raw)
offset = resolve_potential_register_argument_value(cpu, arg2_raw)
address = base + offset

value = cpu.get_register(rdest)
memory.write(address, value, cpu)
```

## Notes
- The memory address is computed as `base + offset`.
- In User mode, `memory.write` performs virtual-to-physical address translation.
- If the virtual address cannot be translated (e.g., page fault), the write is silently ignored.
- `rdest` is the source register, not an address destination.
