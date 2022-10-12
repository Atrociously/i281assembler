; If Statement with Two Conditions (OR)


; C Version
;
; int x=9;
; int min=1;
; int max=8;
; int inRange=1;
;
; if( (x<min) || (x>max) )
;   inRange=0;
;


; Alternative C version that is closer to the assembly code
;
; int x=9;
; int min=1;
; int max=8;
; int inRange=1;
;
; if(x < min)           // First condition
; {
;   Set:
;     inRange=0;
;     goto End;
; }
; else if(max < x)    // Second condition (Note: swapped inequality direction)
; {
;     goto Set;
; }
;
; End:
; 



; Assembly version

.data
x        BYTE     9
min      BYTE     1
max      BYTE     8
inRange  BYTE     1


.code
        LOAD  A, [x]
        LOAD  B, [min]
        LOAD  C, [max]
First:  CMP   A, B                ; x < min ?
        BRGE  Second
Set:    LOADI D, 0
        STORE [inRange], D
        JUMP  End
Second: CMP   C, A                ; max < x ?
        BRGE  End
        JUMP  Set
End:    NOOP
