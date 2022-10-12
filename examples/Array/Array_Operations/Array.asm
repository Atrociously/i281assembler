; Array
;
; Assembly version


.data
array   BYTE    1, 2, 3, 4


.code
        LOADI  A, 0
        STORE  [array + 0], A
        LOAD   B, [array + 1]
        ADDI   B, 5
        STORE  [array + 1], B
        LOAD   C, [array + 2]
        SUBI   C, 1
        STORE  [array + 2], C
        LOAD   D, [array + 3]
        ADD    D, C                ; array[2] is already in C
        STORE  [array + 3], D                


; Register allocation:
; 
; A: array[0]
; B: array[1] 
; C: array[2]
; D: array[3]
;
; This really needs only two registers, 
; but the long version is easier to read.
