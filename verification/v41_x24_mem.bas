$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (24_mem)
DIM a(1 TO 5) AS _MEM, b(0 TO 9) AS _MEM
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
