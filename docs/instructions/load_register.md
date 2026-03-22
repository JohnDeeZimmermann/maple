# Load to Register (LDR)

## Purpose
Loads a value from memory into a register. The memory address is computed by adding an offset to a base address from a register.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x13`: `LDR`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x13` to select load register.

2. `options` (4 bits)
   - **Unused** by this instruction. Set to `0`.

3. `rdest` (4 bits)
   - Specifies the destination register for the loaded value.

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

value = memory.read(address, cpu)
cpu.set_register(rdest, value)
```

## Notes
- The memory address is computed as `base + offset`.
- In User mode, `memory.read` performs virtual-to-physical address translation.
- If the virtual address cannot be translated (e.g., page fault), the read returns 0 and an interrupt may be raised.
- `rdest` is the destination register, not an address source.
