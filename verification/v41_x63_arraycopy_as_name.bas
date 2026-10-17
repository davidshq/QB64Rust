$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (63_arraycopy_as_name)
DIM arraycopy(1 TO 3) AS LONG, b(0 TO 9) AS LONG
_ARRAYCOPY arraycopy() TO b()
PRINT "ok"
SYSTEM
