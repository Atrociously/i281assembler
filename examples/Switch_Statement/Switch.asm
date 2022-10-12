; Switch Statement


; Assembly version

.data
x        BYTE        6
y        BYTE        ?


.code
         LOAD    A, [x]
Zero:    LOADI   C, 0
         CMP     A, C
         BRNE    One
         MOVE    B, A
         SHIFTL  B
         JUMP    End
One:     LOADI   C, 1
         CMP     A, C
         BRNE    Two
         MOVE    B, A
         ADDI    B, 3         
         JUMP    End
Two:     LOADI   C, 2
         CMP     A, C
         BRNE    Default
         MOVE    B, A
         SUBI    B, 1
         JUMP    End
Default: MOVE    B, A
         SHIFTR  B
End:     STORE   [y], B
         NOOP
