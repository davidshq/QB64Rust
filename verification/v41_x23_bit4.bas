$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (23_bit4)
DIM a(1 TO 5) AS _BIT * 4, b(0 TO 9) AS _BIT * 4
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
