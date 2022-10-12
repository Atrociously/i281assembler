; Array plus 5
;
; Assembly version
; (add 5 to all array elements)

.data
array   BYTE    1, 2, 3, 4
N       BYTE    4



.code
        LOADI  A, 0               ; i = 0
For:    LOAD   D, [N]             ; D <- N
        CMP    A, D               ; i < N ?
        BRGE   End                ; if no, exit the for loop
        LOADF  C, [array + A]     ; load array[i]
        ADDI   C, 5               ; add 5
        STOREF [array + A], C     ; store the result back to memory
Iinc:   ADDI   A, 1               ; i++
        JUMP   For                ; repeat the for loop
End:    NOOP


; Register allocation:
; 
; A: i 
; B: <not used>  
; C: temp variable for loading the array elements
; D: N
