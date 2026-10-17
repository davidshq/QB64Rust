$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (13_undeclared_source)
DIM b(0 TO 9) AS LONG
_ARRAYCOPY zz() TO b()
PRINT "ok"
SYSTEM
