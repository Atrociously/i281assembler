; Selection Sort
;
; Assembly version

.data
array   BYTE    -2, 3, 4, 1
last    BYTE    3
i       BYTE    ?


.code
        LOADI  A, 0               ; i = 0
Outer:  STORE  [i], A             ; store i to memory
        LOAD   D, [last]          ; D <- last
        CMP    A, D               ; is i less than last ?
        BRGE   End                ; if no exit the outer loop and the program
        MOVE   C, A               ; maxIndex = i
        MOVE   B, A               ; j = i             
        ADDI   B, 1               ; j++ (the effect of the last two lines: j=i+1)
Inner:  LOAD   D, [last]          ; D <- last
        CMP    B, D               ; j <= last ?
        BRG    Swap               ; if no, jump to the swap
If:     LOADF  A, [array + B]     ; A <- array[j]
        LOADF  D, [array + C]     ; D <- array[maxIndex]
        CMP    D, A               ; is D less than A?
        BRGE   Jinc               ; if no, go to increment j
        MOVE   C, B               ; maxIndex = j
Jinc:   ADDI   B, 1               ; j++
        JUMP   Inner              ; jump to the beginning of the inner loop
Swap:   LOAD   A, [i]             ; restore i from memory
        LOADF  B, [array + A]     ; B <- array[i]
        LOADF  D, [array + C]     ; D <- array[maxIndex]
        STOREF [array + A], D     ; array[i] <- array[maxIndex]
        STOREF [array + C], B     ; array[maxIndex] -< array[i]
Iinc:   ADDI   A, 1               ; i++
        JUMP   Outer              ; jump to the beginning of the outer loop
End:    NOOP



; Register allocation:
; 
; A: i and helper var for the if statement
; B: j and temp 
; C: maxIndex
; D: last, helper var for the swap and for the If
