# IO Read / Write (IOR / IOW)

## Purpose
Transfers data between host memory and IO devices. This instruction provides a mechanism for reading from or writing to hardware devices such as storage, network interfaces, or other peripherals.

**Note:** This instruction is currently a placeholder and not fully implemented.

## Binary Layout
Uses the standard instruction format:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (24) | ARG2 (24)
```

OpCode:
- `0x17`: `IOR/IOW`

## Parameters and Treatment

1. `op_code` (8 bits)
   - Must be `0x17` to select IO read/write.

2. `options` (4 bits)
   - `0`: `IOR` - read from IO device into host memory
   - `1`: `IOW` - write from host memory to IO device
   - Other: No operation

3. `rdest` (4 bits)
   - For `IOR`: destination register where the IO device identifier is stored
   - For `IOW`: source register containing the IO device identifier

4. `arg1_raw` (24 bits)
   - The host memory address (source for write, destination for read).
   - Resolved by `resolve_potential_register_argument_value`.

5. `arg2_raw` (24 bits)
   - The length of data to transfer in bytes.
   - Resolved by `resolve_potential_register_argument_value`.

## Execution Flow
```text
// Currently not implemented
cpu.set_register(rdest, 0)
pc += 1
```

## Notes
- This instruction is a placeholder and currently performs no actual IO operations.
- For certain IO devices (e.g., storage devices), direct IO operations may be restricted within a CAR (Contiguous Address Range).
- The IO device identifier specifies which hardware device to communicate with.
- Full implementation requires IO device registration and management infrastructure.
