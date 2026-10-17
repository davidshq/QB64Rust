$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (16_long_unsigned)
DIM a(1 TO 5) AS LONG, b(0 TO 9) AS _UNSIGNED LONG
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
