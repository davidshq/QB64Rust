$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (11_scalar_source)
DIM x AS LONG, b(0 TO 9) AS LONG
_ARRAYCOPY x() TO b()
PRINT "ok"
SYSTEM
