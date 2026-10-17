$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (59_lower_case)
DIM a(1 TO 5) AS LONG, b(0 TO 9) AS LONG
_arraycopy a() to b()
PRINT "ok"
SYSTEM
