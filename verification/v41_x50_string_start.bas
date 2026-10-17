$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (50_string_start)
DIM a(1 TO 5) AS LONG, b(0 TO 9) AS LONG
_ARRAYCOPY a(1 TO 2) TO b("0")
PRINT "ok"
SYSTEM
