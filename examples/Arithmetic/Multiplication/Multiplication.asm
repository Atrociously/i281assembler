; Multiplication


; C Version
;
; int x=3;
; int z;
;
; z = x*5;


; Assembly version

.data
x        BYTE        3
z        BYTE        ?


.code
        LOAD    A, [x]
        MOVE    C, A        ; z=x;
        MOVE    B, A        ; B=x;
        SHIFTL  B           ; B=2x
        SHIFTL  B           ; B=4x
        ADD     C, B        ; C=4x+x
        STORE   [z], C      ; update the memory for z


; Register allocation
;
; A: x
; B: temporary results for 2x and 4x
; C: z
; D: <not used>
