$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (01_no_to)
DIM a(1 TO 5) AS LONG, b(0 TO 9) AS LONG
_ARRAYCOPY a() b()
PRINT "ok"
SYSTEM
