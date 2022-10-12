; Do Loop
;
; Add the numbers from 1 to 5 using a do loop.


; C Version 
;
; int N=5;
; int sum;
; int i;
; 
; i=0;
; sum=0;
; 
; do
; {
;     i++;
;     sum+=i;
; }while( i < N );
; 


; Assembly version

.data
N       BYTE      5
sum     BYTE      ?

; i is optimized to register A


.code
        LOADI A, 0           ; i = 0
        LOADI B, 0           ; sum=0
        LOAD  D, [N]         ; register D = N
Do:     ADDI  A, 1           ; i++
        ADD   B, A           ; sum+=i
        CMP   D, A           ; N > i ? (register ordering is swapped)
        BRG   Do             ; if true, jump to Do
End:    STORE [sum], B       ; store sum to memory


; Register allocation:
; 
; A: i 
; B: sum  
; C: <not used>
; D: N
