; While Loop
;
; Add the numbers from 1 to 5 using a while loop.



; C Version
;
; int N=5;
; int sum;
; int i;
; 
; i=1;
; sum=0;
; 
; while( i <= N )
; {
;    sum+=i;
;    i++;
; }



; Assembly Version

.data
N       BYTE     5
sum     BYTE     ?

; i is optimized to register A


.code
        LOADI A, 1         ; i = 1
        LOADI B, 0         ; sum=0
        LOAD  D, [N]       ; register D = N
While:  CMP   A, D         ; i <= N ?
        BRG   End          ; if no, exit the while loop
        ADD   B, A         ; sum+=i
        ADDI  A, 1         ; i++
        JUMP  While        ; next iteration
End:    STORE [sum], B     ; store sum to memory


; Register allocation:
; 
; A: i 
; B: sum  
; C: <not used>
; D: N
