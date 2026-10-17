$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (17_single_double)
DIM a(1 TO 5) AS SINGLE, b(0 TO 9) AS DOUBLE
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
