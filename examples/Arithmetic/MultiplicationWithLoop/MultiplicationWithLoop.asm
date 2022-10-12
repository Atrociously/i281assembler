; Multiplication With Loop



; C Version
;
; int x=3;
; int z;
;
; z = x*5;

; alternative C version that models the assembly code
;
; int x=3;
; int z=0;
;
; for(int i=0; i<5; i++){
;     z+=x;
; }


; Assembly version
;
; The CPU does not have OPCODEs for multiplication.
; Therefore, it is emulated with repeated addition.

.data
x        BYTE        3
z        BYTE        ?


.code
        LOAD  A, [x]
        LOADI C, 0        ; z=0
        LOADI B, 0        ; i=0
        LOADI D, 5        ; sentinel value
For:    CMP   B, D        ; i<5?
        BRGE  End         ; if(i>=5), exit for loop
        ADD   C, A        ; z+=x
        ADDI  B, 1        ; i++
        JUMP  For         ; jump to For loop
End:    STORE [z], C      ; update the z value in memory

; Register allocation
;
; A: x
; B: i (optimized to register)
; C: z
; D: 5
