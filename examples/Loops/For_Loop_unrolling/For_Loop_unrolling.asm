; For Loop Unrolling


; C Version (original)
;
; int N=3;
; int i, sum;
;
; sum=0;
; for(i=1; i<=N; i++)
;   sum+=i;


; C Version (with loop unrolling)
; int i, sum;
; 
; sum=0;
; i=1;        // i<-1
; sum+=i;
; i++;        // i<-2
; sum+=i;
; i++;        // i<-3
; sum+=i;



; Assembly version
; (the code is shorter when the loop is unrolled)

.data
sum        BYTE        ?
; i is optimized to a register

.code
        LOADI B, 0       ; sum=0
        LOADI A, 1       ; i=1
        ADD   B, A       ; sum+=i
        ADDI  A, 1       ; i++
        ADD   B, A       ; sum+=i
        ADDI  A, 1       ; i++
        ADD   B, A       ; sum+=i
        STORE [sum], B   ; update the memory for sum



; Register allocation:
; 
; A: i 
; B: sum 
; C: <not used>
; D: <not used>
