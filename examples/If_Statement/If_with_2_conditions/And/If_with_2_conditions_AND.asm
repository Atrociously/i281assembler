; If Statement with Two Conditions (AND)


; C Version
;
; int x=5;
; int min=1;
; int max=8;
; int inRange=0;
;
; if( (x>=min) && (x<=max) )
;   inRange=1;
;
;

; Alternative C version that is closer to the assembly code
;
; int x=5;
; int min=1;
; int max=8;
; int inRange=0;
;
; if(min<=x)      // note that the inequality direction is swapped
; {
;   if(x<=max)
;   {
;      inRange=1;
;   }
; }

; Assembly version

.data
x        BYTE        5
min      BYTE        1
max      BYTE        8
inRange  BYTE        0

.code
        LOAD  A, [x]
        LOAD  B, [min]
        LOAD  C, [max]
        CMP   B, A                ; min <= x ?
        BRG   End
        CMP   A, C                ; x <= max ?
        BRG   End
        LOADI D, 1
        STORE [inRange], D
End:    NOOP
