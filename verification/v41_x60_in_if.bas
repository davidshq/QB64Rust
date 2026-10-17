$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (60_in_if)
DIM a(1 TO 5) AS LONG, b(0 TO 9) AS LONG
IF 1 THEN _ARRAYCOPY a() TO b() ELSE PRINT "no"
PRINT "ok"
SYSTEM
