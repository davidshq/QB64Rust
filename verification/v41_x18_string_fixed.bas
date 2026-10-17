$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (18_string_fixed)
DIM a(1 TO 5) AS STRING, b(0 TO 9) AS STRING * 4
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
