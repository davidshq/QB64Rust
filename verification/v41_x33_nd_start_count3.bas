$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (33_nd_start_count3)
DIM m(1 TO 2, 1 TO 3) AS LONG, n(0 TO 3, 0 TO 4) AS LONG
_ARRAYCOPY m(1 TO 2, 1 TO 2) TO n(0, 0, 0)
PRINT "ok"
SYSTEM
