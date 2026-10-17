$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (48_const_source)
CONST k = 5
DIM b(0 TO 9) AS LONG
_ARRAYCOPY k() TO b()
PRINT "ok"
SYSTEM
