$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (26_whole_dims)
DIM a(1 TO 5) AS LONG, n(0 TO 3, 0 TO 4) AS LONG
_ARRAYCOPY a() TO n()
PRINT "ok"
SYSTEM
