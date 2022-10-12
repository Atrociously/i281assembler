; Insertion Sort
;
; Assembly version

.data
array    BYTE    2, 3, 4, 1
N        BYTE    4

; These are mapped to registers:
; i         BYTE      ?
; j         BYTE      ?
; INS       BYTE      ?


.code
            LOADI  A, 1              ; i = 1
For:        LOAD   D, [N]            ; D <- N
            CMP    A, D              ; i < N ?
            BRGE   End               ; if no, exit the for loop
            MOVE   B, A              ; j = i             
            LOADF  C, [array + A]    ; INS = array[i]
While:      LOADI  D, 0              ; D <- 0
            CMP    D, B              ; 0 < j ?
            BRGE   Insertion         ; if no, go to Insertion
            LOADF  D, [array+B-1]    ; D <- array[j-1]
            CMP    D, C              ; array[j-1] < INS ?
            BRGE   Insertion         ; if no, go to Insertion
Shuffle:    STOREF [array + B], D    ; array[j] = array[j-1]
            SUBI   B, 1              ; j--
            JUMP   While             ; repeat the while loop
Insertion:  STOREF [array + B], C    ; array[j] = INS
Iinc:       ADDI   A, 1              ; i++
            JUMP   For               ; repeat the for loop
End:        NOOP


; Register allocation:
; 
; A: i 
; B: j  
; C: INS
; D: helper variable: N, 0, array[j-1]
