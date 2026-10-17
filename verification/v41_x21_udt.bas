$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (21_udt)
TYPE p
    x AS LONG
END TYPE
DIM a(1 TO 5) AS p, b(0 TO 9) AS p
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
