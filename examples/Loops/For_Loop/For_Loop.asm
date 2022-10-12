; For Loop


; C Version
;
; Add the numbers from 1 to 5 using a for loop.
;
; int N=5;
; int i, sum;
;
; sum=0;
; for(i=1; i<=N; i++) {
;   sum+=i;
; }


; Assembly version


.data
N        BYTE    5
i        BYTE    ?
sum      BYTE    ?


.code
        LOADI  B, 0        ; sum=0
        LOADI  A, 1        ; i=1
        LOAD   D, [N]      ; register D = N
Loop:   CMP    A, D        ; i<=N ?
        BRG    End         ; exit if i>N
Add:    ADD    B, A        ; sum+=i
        ADDI   A,1         ; i++
        JUMP   Loop        ; next iteration
End:    STORE  [sum], B    ; update the memory for sum


; Register allocation:
; 
; A: i 
; B: sum 
; C: <not used>
; D: N
