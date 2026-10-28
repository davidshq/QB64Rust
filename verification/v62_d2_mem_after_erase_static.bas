$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 8, the block of a static array after ERASE (which only clears
' it): _MEMEXISTS, then a _MEMPUT through it and the element it wrote.
ON ERROR GOTO h
DIM m AS _MEM
DIM s(5) AS LONG
s(0) = 9
m = _MEM(s())
ERASE s
PRINT "1."; _MEMEXISTS(m); s(0)
_MEMPUT m, m.OFFSET, 4 AS LONG
PRINT "2."; s(0)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
