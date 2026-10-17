$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (69_integer64_offset)
DIM a(1 TO 5) AS _INTEGER64, b(0 TO 9) AS _OFFSET
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
