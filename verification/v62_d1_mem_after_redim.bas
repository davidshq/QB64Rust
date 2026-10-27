$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 8, the block of a dynamic array after REDIM: _MEMEXISTS, and
' the error a _MEMPUT through it gives (errors 300 to 315 are not trapped).
ON ERROR GOTO h
DIM m AS _MEM
REDIM d(5) AS LONG
m = _MEM(d())
PRINT "1."; _MEMEXISTS(m)
REDIM d(6) AS LONG
PRINT "2."; _MEMEXISTS(m)
_MEMPUT m, m.OFFSET, 1 AS LONG
PRINT "3. after _MEMPUT"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
