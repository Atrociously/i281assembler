; C Version of the program
;
; #include<stdio.h>
;
; int x = 2;
; int y;
; int result;
;
; //Simple Addition with User input for the second variable (y)
; void main()
; {
;        scanf("%d", &y);
;        result = x + y;
;        //printf("%d", result);
; }



; Assembly version

.data
x         BYTE 2
y         BYTE ?
result    BYTE ?

.code

        LOAD   A, [x]            ; Register A <- [x]
        INPUTD [y]               ; [y] <- Input Value
        LOAD   B, [y]            ; Register B <- [y]
        ADD    A, B              ; A <- A + B
        STORE  [result], A       ; result <- Register A



; Register allocation:  
; 
; A: x
; B: Input Value, y
; C: not used
; D: not used

; Notes: The scanf is not implemented as a function. Instead the code uses the
; INPUTD opcode to read a value from the input switches and store it in data memory.
; This still accomplishes user input, but there is no function call since this CPU
; does not support function calls (because it does not have a stack). 

