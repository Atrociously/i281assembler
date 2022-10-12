; Linear Search



.data
array   BYTE   5, 2, 7, 3, 6, 1
found   BYTE   0
index   BYTE   ?
N       BYTE   6
key     BYTE   5
i       BYTE   ?

.code
        LOADI  A, 0             ; i=0
        LOAD   B, [N]           ; Register B <- [N]
        LOAD   D, [key]         ; Register D <- [key]
For:    CMP    A, B             ; Compare A with B
        BRGE   End              ; if i >= N exit the loop
Step:   LOADF  C, [array+A]     ; Register C <- array[i]
        CMP    C, D             ; Compare C with D
        BRNE   Iinc
Match:  LOAD   C, [found]       ; Load C <- [found]
        ADDI   C, 1             ; C <- C + 1
        STORE  [found], C       ; Store [found] <- C
        STORE  [index], A
        JUMP   End
Iinc:   ADDI   A, 1             ; i++
        JUMP   For              ; Jump to For
End:    NOOP


; Register allocation:
; 
; A: i 
; B: N 
; C: array[i], found
; D: key
