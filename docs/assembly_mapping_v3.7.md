# Assembly to machine code mapping for the i281 CPU
The CPU has 26 OPCODES. Their abbreviations and full names are listed below.
This is a modification of some documentation I got into markdown format

| OpCode   | Naming Scheme                                                  | Machine Code        | Meaning                                                                     |
| -------- | -------------------------------------------------------------- | ------------------- | ----------------------------------------------------------------            |
| NOOP     | **NO OP**eration                                               | 0000_DD_DD_DDDDDDDD | Do nothing                                                                  |
| INPUTC   | **INPUT** into **C**ode memory                                 | 0001_DD_00_CADDRESS | CODE at [Address + Const]      = INPUTVAL from switches SW15-SW0            |
| INPUTCF  | **INPUT** into **C**ode memory with o**F**fset                 | 0001_DD_01_CADDRESS | CODE at [Address + RX + Const] = INPUTVAL from switches SW15-SW0            |
| INPUTD   | **INPUT** into **D**ata memory                                 | 0001_DD_10_CADDRESS | DATA at [Address + Const]      = INPUTVAL from switches SW7-SW0             |
| INPUTDF  | **INPUT** into **D**ata memory with o**F**fset                 | 0010_RX_RY_00000000 | DATA at [Address + RX + Const] = INPUTVAL from switches SW7-SW0             |
| MOVE     | **MOVE** (i.e. copy) the contents of one register into another | 0011_RX_DD_IMMEDVAL | RX = RY + 0 *copies value of RY into RX*                                    |
| LOADI    | **LOAD** **I**mmediate value into register                     | 0011_RX_DD_POINTVAL | RX = IMMEDVAL                                                               |
| LOADP    | **LOAD** **P**ointer address into register (not implemented)   | 0100_RX_RY_DDDDDDDD | RX = IMMEDVAL *pointer operation handled by compiler*                       |
| ADD      | **ADD** two regisers                                           | 0101_RX_DD_IMMEDVAL | RX = RX + RY                                                                |
| ADDI     | **ADD** an **I**mmediate value to a register value             | 0110_RX_RY_DDDDDDDD | RX = RX + IMMEDVAL                                                          |
| SUB      | **SUB**tract two registers                                     | 0111_RX_DD_IMMEDVAL | RX = RX - RY                                                                |
| SUBI     | **SUB**tract an **I**mmediate value from a register value      | 1000_RX_DD_DADDRESS | RX = RX - IMMEDVAL                                                          |
| LOAD     | **LOAD** from a data memory address into a register            | 1001_RX_RY_DADDRESS | RX = [Address + Const]                                                      |
| LOADF    | **LOAD** with an o**F**fset specified by another register      | 1010_RX_DD_DADDRESS | RX = [Address + RY + Const]                                                 |
| STORE    | **STORE** a register into a data memory address                | 1011_RX_RY_DADDRESS | [Address + Const] = RX                                                      |
| STOREF   | **STORE** with an o**F**fset specified by another register     | 1100_RX_D0_DDDDDDDD | [Address + RY + Const] = RX                                                 |
| SHIFTL   | **SHIFT** all bits in a register **L**eft                      | 1100_RX_D1_DDDDDDDD | RX = RX << 1                                                                |
| SHIFTR   | **SHIFT** all bits in a register **R**ight                     | 1100_RX_D1_DDDDDDDD | RY = RX >> 1                                                                |
| CMP      | **C**o**MP**are the values in two registers                    | 1101_RX_RY_DDDDDDDD | If RX - RY = 0, set the ZERO-FLAG. If RX-RY<0, set the NEGATIVE FLAG        |
| JUMP     | **JUMP** unconditionally to a specified address                | 1110_DD_DD_PCOFFSET | PC = PC + 1 + PCOFFSET *PCOFFSET is signed 2's complement*                  |
| BRE      | **BR**anch if **E**qual                                        | 1111_DD_00_PCOFFSET | PC = PC + 1 + PCOFFSET if condition *Equal* is true                         |
| BRZ      | **BR**anch if **Z**ero                                         | 1111_DD_00_PCOFFSET | PC = PC + 1 + PCOFFSET if condition *Zero* is true *identical to BRE*       |
| BRNE     | **BR**anch if **N**ot **E**qual                                | 1111_DD_01_PCOFFSET | PC = PC + 1 + PCOFFSET if condition *Not Equal* is true                     |
| BRNZ     | **BR**anch if **N**ot **Z**ero                                 | 1111_DD_01_PCOFFSET | PC = PC + 1 + PCOFFSET if condition *Not Zero* is true  *identical to BRNE* |
| BRG      | **BR**anch if **G**reater                                      | 1111_DD_10_PCOFFSET | PC = PC + 1 + PCOFFSET if condition *Greater than* is true                  |
| BRGE     | **BR**anch if **G**reater than or **E**qual                    | 1111_DD_11_PCOFFSET | PC = PC + 1 + PCOFFSET if condition *Greater than or equal* is true         |

## Some notes on the assembly to machine code mapping

Many instructions use or modify the value of one of the CPU registers. 
For these instructions, the address of the specific register is denoted with RX.
In this case, RX is a 2-bit value that denotes one of the four registers of the CPU:
        RX=00 selects register A, 
        RX=01 selects register B, 
        RX=10 selects register C, 
        RX=11 selects register D. 


Several instructions use the values of two registers (and usually modify one of them). 
In those cases the addresses of the two registers are denoted with RX and RY. 
Similarly to RX, RY is a 2-bit value that denotes one of the four registers of the CPU:
        RY=00 selects register A, 
        RY=01 selects register B, 
        RY=10 selects register C, 
        RY=11 selects register D. 


d - Denotes a don't care bit that is ignored by the hardware. 
    The assembler, however, maps all d bits to 0.


The vertical lines | are used for better visual separation between the bits.
They are not part of the machine language. 


The six branch instructions are clustered into four groups. They all have the same 1111 opcode, 
but the two least significant bits in the first byte determine the type of branch. 
The general format for a branch instruction is the following:
```
BR??       1111 |dd|CC||PCOFFSET|     ; PC = PC + PCOFFSET if condition CC is true

                    CC = 00: BRE  or BRZ 
                    CC = 01: BRNE or BRNZ
                    CC = 10: BRG 
                    CC = 11: BRGE
```
At the machine-language level, the instruction BRE is indistinguishable from BRZ. 
These aliased instructions are handled by the assembler.  Similarly, BRNE is 
aliased with BRNZ.


The program counter offset (i.e., PCOFFSET) is an 8-bit value stored in 2's complement. 
Thus, it can be either positive or negative.  If the brach condition is true, then the 
value of PCOFFSET is added to the Program Counter (PC). Negative values move the 
PC back, positive values advance it forward. Because in the hardware implementation the PC 
is always incremented by 1 after each instruction, the offset value needs to be corrected accordingly. 
The JUMP instruction implements an unconditional jump. Its PCOFFSET value needs to be similarly adjusted. 


There are four INPUT instructions that allow the user to enter a value from the switches on the board. 
They differ only by the last two bits in the first byte:
```
INPUT??    0001 |dd|CC||MADDRESS|     ; [MADDRESS]=INPUTVAL from switches

                    CC = 00: INPUTC
                    CC = 01: INPUTCF
                    CC = 10: INPUTD
                    CC = 11: INPUTDF
```
INPUTC and INPUTCF read a 16-bit value (from SW15-SW0) and store it in the code memory at the given address.

INPUTD and INPUTDF read an 8-bit value (from SW7-SW0) and store it in the data memory at the given address.


The instructions SHIFTL and SHIFTR differ only by the least significant bit in the first byte. If it is
equal to 0, then the shift is to the left. If it is equal to 1, then the value of the register is shifted
to the right. In both cases, the bit that is shifted out is stored in the overflow flag in the flags register.


The CPU has two memories: one for data and one for code. Some of the opcodes need to store 
an address into one of these memories in the second byte of the instruction. In the 
description above these are called DADDRESS and CADDRESS, which are defined as follows: 

DADDRESS is an address into the data memory,

CADDRESS is an address into the code memory.


In the opcodes these are hardcoded values that are computed by the assembler when it emits the machine code. 
In the assembly language (but not in the machine code) there is some flexibility in the way the address
is specified. In particular, an optional constant offset can be added to the address. The assembler will perform 
the addition and will store the result in the second byte of the opcode. For example, 

   [Address] = DADDRESS.

or

   [Address + Const] = DADDRESS.

Opcodes that end in 'F' allow for another offset to be added to the address, in addition to the optional
constant offset. That variable offset value is stored in one of the registers (specified by the two RX bits). 
In this case, however, the register value offset is added at run time through the ALU.  Thus, for 

   [Address + RX]   the DADDRESS is equal to Address. The value of register RX is added at runtime.

or for

   [Address + RX + Const]   the DADDRESS is equal to [Address + Const]. The value of register RX is added at runtime.
