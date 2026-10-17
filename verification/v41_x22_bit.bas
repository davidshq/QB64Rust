$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (22_bit)
DIM a(1 TO 5) AS _BIT, b(0 TO 9) AS _BIT
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
