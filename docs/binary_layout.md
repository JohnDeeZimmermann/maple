# Binary Layout
This document describes the binary layout of each instruction.
### General Layout
We have 64-bit available for each instruction. The general layout we look for is:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (23 + 1) | ARG2(23 + 1)
```
This is made for simplicity and it does have drawbacks to standardize it like this. 
Certain Operations will differ from this layout.

`OPTIONS` contains additional information on the instruction. For example which _compare_ command should be executed.
Each argument also contains an additional bit (to the right) which determines whether the argument is stored in a register or directly.

For most instructions, direct arguments are decoded as unsigned payloads.
Integer math instructions (`ADDI`, `SUBI`, `MULI`, `DIVI`) are the exception: they interpret bits 23..1 as a signed 23-bit immediate and sign-extend it to 64 bits.

##### Example
`ADDI r1, r2, #16` would be represented as
```
00000010 0000 0001 00000000000000000000010 1 00000000000000000010000 0
OPCODE   OPT  DEST ARG1 (As register)      R ARG2 (Directly)         R
```

`ADDI r1, #-5, #2` stores `arg1` as the 23-bit two's-complement payload for `-5`, plus the trailing direct/register bit:
```
00000010 0000 0001 11111111111111111111011 0 00000000000000000000010 0
OPCODE   OPT  DEST ARG1 (Directly, signed) R ARG2 (Directly)         R
```
### Move, Move Not
```
OPCODE (8) | OPTIONS (1) | RDEST (4) | SIGN(1) | VALUE (49 + 1)
```
The sign bit represents the MSB of the resulting value stored. An OR operation is performed on the MSB, regardless whether a register value is imported or not.

_To achieve larger values, one could always adjust the offset values or perform add operations. Our implementation does however only support 32-bit memory addresses ($2^{32}$ words or $8*2^{32}=2^{36}$ bytes_)
##### Example
`MOV r2, #128` would be translated to:
```
00000001 0 0010 00000000000000000000000000000000000000000010000000 0
OPCODE OPT DEST VALUE (Directly)                                   R
```
### Condition Result Register
We use the `cr` register to store information about the last mathematical operation or comparison. 
A comparison of `A and B` will simply be populated by the mathematical properties of `A - B`. 

Additionally, it is used to store additonal system information.
These are the flags set:
```
WIP... 000... | PARITY(1) | NEGATIVE(1) | ZERO(1) | OVERFLOW (1) 
```

### System Info Register

The first 16 bits of the `sy` register contain the address of the interrupt table (`interrupt_table_base`).
In case of an interrupt, the program counter is set to the address specified + the code of the interrupt.

The next 16 bits of the `sy` register contain the length of the interrupt table (`interrupt_table_length`). 
If an interrupt is raised with a code larger than `interrupt_table_length`, an interrupt is raised. 

When an interrupt is raised, the current program counter (physical) is set to the last 32 bits.

```
INTERRUPT_TABLE_BASE(16) | INTERRUPT_TABLE_LENGTH (16) | OLD_PC (32)
```

This register can only be modified when in kernel mode.
