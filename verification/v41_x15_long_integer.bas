$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (15_long_integer)
DIM a(1 TO 5) AS LONG, b(0 TO 9) AS INTEGER
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
