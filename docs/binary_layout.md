# Binary Layout
This is a description on how the binary layout of each instruction is supposed to look like. 
### General Layout
We have 64-bit available for each instruction. The general layout we look for is:
```
OPCODE (8) | OPTIONS (4) | RDEST (4) | ARG1 (23 + 1) | ARG2(23 + 1)
```
This is made for simplicity and it does have drawbacks to standardize it like this. 
Certain Operations will differ from this layout.

`OPTIONS` contains additional information on the instruction. For example which _compare_ command should be executed.
Each argument also contains an additional bit (to the right) which determines whether the argument is stored in a register or directly.

##### Example
`ADDI r1, r2, #16` would be represented as
```
00000010 0000 0001 00000000000000000000010 1 00000000000000000010000 0
OPCODE   OPT  DEST ARG1 (As register)      R ARG2 (Directly)         R
```
### Move, Move Not
In order to store larger values (e.g. memory addresses exceeding 23 bits in size),MOV and MVN have a slightly different layout. 
```
OPCODE (8) | OPTIONS (1) | RDEST (4) | VALUE (50 + 1)
```
_To achieve larger even larger addresses, one could always adjust the offset values or perform add operations. Our implementation does however only support 32-bit memory addresses ($2^{32}$ words or $8*2^{32}=2^{36}$ bytes_)
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
000000 | STORAGE-DVC (4) | GPU-DVC(4) | IO-DEVICES (4) | INTERRUPT-CODE (8) | INTERRUPT-RESULT (32) | SYS-STATUS(1) | PARITY(1) | EVEN(1) | NEGATIVE(1) | ZERO(1) | OVERFLOW (1) 
```