; If-Else Statement


; C Version
;
; int x=3;
; int y=5;
; int z;
;
; if(x<y)
;   z=x;
; else
;   z=y;


; Assembly version

.data
x	BYTE	3
y	BYTE	5
z	BYTE	?


.code
	LOAD  A, [x]
	LOAD  B, [y]
If:	CMP   A, B
	BRGE  Else
	STORE [z], A
	JUMP  End
Else:	STORE [z], B
End:	NOOP
