$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (14_undeclared_target)
DIM a(1 TO 5) AS LONG
_ARRAYCOPY a() TO zz()
PRINT "ok"
SYSTEM
