; Arithmetic

; C Version
;
; int x=2;
; int y=3;
; int z;
;
; z = x+y;


; Assembly version

.data
x        BYTE        2
y        BYTE        3
z        BYTE        ?


.code
        LOAD  A, [x]
        LOAD  B, [y]
        MOVE  C, A        ; z=x;
        ADD   C, B        ; z+=y;
        STORE [z], C


; Register allocation
;
; A: x
; B: y
; C: z
; D: <not used>
