; Binary Search


.data
array      BYTE 2, 4, 5, 7, 8, 9
found      BYTE 0
mid        BYTE ?
low        BYTE 0
high       BYTE 5
key        BYTE 4


.code
           LOAD    A, [low]         ; Register A <- [low]
           LOAD    C, [high]        ; Register 
           LOAD    D, [key]         ; Register D <- [key]
While:     CMP     A, C             ; Compare A with C
           BRG     End              ; exit the loop if low > high 
Step:      MOVE    B, A             ; B <- low
           ADD     B, C             ; B <- B + hight
           SHIFTR  B                ; B <- B / 2
If:        STORE   [mid], B         ; Store [mid] <- B 
           LOADF   B, [array+B]     ; B = array[mid]
           CMP     B, D             ; Compare array[mid] with key
           BRE     Match            ; if they are equal we found a match
           BRGE    AdjHigh          ; reuses the flags to handle the If-Else
AdjLow:    LOAD    A, [mid]         ; low = mid
           ADDI    A, 1             ; low = mid + 1
           JUMP    While            ; Jump to While
AdjHigh:   LOAD    C, [mid]         ; high = mid
           SUBI    C, 1             ; high = mid - 1
           JUMP    While            ; Jump to While
Match:     LOAD    B, [found]       ; Load B <- [found]
           ADDI    B, 1             ; B <- B + 1
           STORE   [found], B       ; Store [found] <- B; implicit break
End:       NOOP                     ; Do nothing


; Register allocation:
; 
; A: low 
; B: mid or array[mid] or found
; C: high
; D: key

; Notes: This code optimizes the three-part if by taking advantage of the 
; almost identical check in the if and the if-else. By reusing the CPU flags
; this is done in assembly in the opposite order: else, else-if, if.
;
; In this version low and high are mapped to registers and their values are
; not available (stored to data memory) after the end of the program. 
