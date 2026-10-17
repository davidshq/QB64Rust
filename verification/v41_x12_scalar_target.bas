$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (12_scalar_target)
DIM a(1 TO 5) AS LONG, x AS LONG
_ARRAYCOPY a() TO x()
PRINT "ok"
SYSTEM
