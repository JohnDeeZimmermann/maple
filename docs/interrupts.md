# Interrupts

Interrupts are raised with interrupt codes which point to an address in an interrupt table. 
When an interrupt is raised, the program counter `pc` is set to the value found at the
`interrupt_table_base` added by `interrupt_table_length`, specified by the system info register `sy`. 

## System Interrupt Codes


| Code   | Name                          | Reason                                                                                |
|--------|-------------------------------|---------------------------------------------------------------------------------------|
| `0000` | Invalid interrupt code        | An interrupt was raised with a code exceeding the length of the interrupt table       | 
| `0001` | Division by zero              | A division by zero was executed                                                       | 
| `0002` | Invalid OP Code               | An instruction was executed with an invalid OPCode                                    | 
| `0003` | Timeout                       | Occurs when the pre-defined timout was exceed, if specified.                          | 
| `0004` | Page Fault                    | Raised when processes try to access unreserved memory addresses.                      | 
| `0005` | Illegal Register Modification | Raised when a restricted register is modified in user mode.                           | 
| `0006` | Illegal Direct Argument       | Raised when a direct value is passed where only a reference to a register is allowed. | 

