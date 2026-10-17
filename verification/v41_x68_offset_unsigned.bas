$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (68_offset_unsigned)
DIM a(1 TO 5) AS _OFFSET, b(0 TO 9) AS _UNSIGNED _OFFSET
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
