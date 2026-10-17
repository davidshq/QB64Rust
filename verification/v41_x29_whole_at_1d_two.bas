$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (29_whole_at_1d_two)
DIM a(1 TO 5) AS LONG, b(0 TO 9) AS LONG
_ARRAYCOPY a() TO b(0, 1)
PRINT "ok"
SYSTEM
