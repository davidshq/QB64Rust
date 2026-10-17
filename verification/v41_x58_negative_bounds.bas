$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (58_negative_bounds)
DIM a(-5 TO -1) AS LONG, b(-9 TO 0) AS LONG
_ARRAYCOPY a(-4 TO -2) TO b(-9)
PRINT "ok"
SYSTEM
