$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (61_after_colon)
DIM a(1 TO 5) AS LONG, b(0 TO 9) AS LONG
PRINT "x";: _ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
