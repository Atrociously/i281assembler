; Struct
;
; Assembly version


.data
point   BYTE    ?, ?, ?


.code
        LOADI A, 2
        STORE [point+0], A
        LOADI B, 3
        STORE [point+1], B
        ADD A, B
        STORE [point+2], A

; Register allocation:
; 
; A: 2 and 2+3
; B: 3 
; C: not used
; D: not used
