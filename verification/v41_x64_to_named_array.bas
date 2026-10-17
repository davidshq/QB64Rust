$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (64_to_named_array)
DIM a(1 TO 5) AS LONG, too(0 TO 9) AS LONG
_ARRAYCOPY a() TO too()
PRINT "ok"
SYSTEM
