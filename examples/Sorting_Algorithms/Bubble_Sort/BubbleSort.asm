; Bubble Sort


; C Version
;
; int array[] = {7, 3, 2, 1, 6, 4, 5, 8};
; int last = 7; // last valid index in the array
; int temp;
; int i,j;
; 
; int main()
; {
;    for (i = 0; i < last; i++) 
;        for (j = 0; j < last-i; j++) 
;               if (array[j] > array[j+1]){
;                      temp = array[j];
;                      array[j] = array[j+1];
;                      array[j+1] = temp;
;               }
; }



; Assembly version

.data
array   BYTE 7, 3, 2, 1, 6, 4, 5, 8
last    BYTE 7
temp    BYTE ?

.code
        LOADI  A, 0                  ; i = 0;
Outer:  LOAD   D, [last]             ; Load last into D
        LOADI  B, 0                  ; j = 0;
        CMP    A, D                  ; i < last
        BRGE   End                   ; If i >= last break out of the outer loop
Inner:  LOAD   D, [last]             ; Re-Load last into D (this register is shared)
        SUB    D, A                  ; D = D - A  (i.e., D = last - i)
        CMP    B, D                  ; j < last - i
        BRGE   Iinc                  ; If j >= last-i  branch to Iinc
If:     LOADF  C, [array+B]          ; C = array[j]
        LOADF  D, [array+B+1]        ; D = array[j+1] (compiler adds 1 to addr. of array)
        CMP    D, C                  ; if array[j+1] < array[j]  (switched direction)
        BRGE   Jinc
Swap:   STOREF [array+B], D
        STOREF [array+B+1], C
Jinc:   ADDI   B, 1                  ; j++
        JUMP   Inner
Iinc:   ADDI   A, 1                  ; i++
        JUMP   Outer
End:    NOOP                         ; Do nothing



; Register allocation:  
; 
; A: i
; B: j
; C: array[j]
; D: last, array[j+1]

; Note that i and j are optimized away. 
; They exist only in registers, not in the data memory.
