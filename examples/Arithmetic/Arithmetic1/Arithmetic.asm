; Arithmetic


; C Version
;
; int x=2;
; int z;
;
; z = x+3;


; Assembly version

.data
x        BYTE        2
z        BYTE        ?


.code
        LOAD  A, [x]
        MOVE  C, A        ; z=x;
        ADDI  C, 3        ; z+=3;
        STORE [z], C      ; update the memory for z


; Register allocation
;
; A: x
; B: <not used>
; C: z
; D: <not used>
