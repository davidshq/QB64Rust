$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (19_fixed_string)
DIM a(1 TO 5) AS STRING * 4, b(0 TO 9) AS STRING
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
