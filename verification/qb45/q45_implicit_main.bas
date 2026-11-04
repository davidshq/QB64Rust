' QB 4.5 (DIVERGENCES-QB45.md Q-017): an array used without DIM in the main module. Whether
' BC warns "Array not dimensioned" there as it does in a SUB (q45_implicit_sub), and the
' array's bounds and first value. QB64pe gives no message and an array 0 to 10.
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
PRINT #1, "first read:"; z(2)
z(2) = 5
PRINT #1, "z(2), LBOUND, UBOUND:"; z(2); LBOUND(z); UBOUND(z)
PRINT #1, "z(11) = 1:";
z(11) = 1
PRINT #1, "end"
CLOSE #1
SYSTEM
h:
PRINT #1, "[error"; ERR; "]"
RESUME NEXT
