; If Greater than or Equal


; C Version
;
; int x=3;
; int y=5;
; int z=0;
;
; if(x>=y)
;   z=x;


; Assembly version

.data
x       BYTE    3
y       BYTE    5
z       BYTE    0


.code
        LOAD  A, [x]
        LOAD  B, [y]
        CMP   B, A        ; these are swapped
        BRG   End
        STORE [z], A
End:    NOOP
