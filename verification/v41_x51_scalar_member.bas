$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (51_scalar_member)
TYPE p
    x AS LONG
END TYPE
DIM v AS p, b(0 TO 9) AS LONG
_ARRAYCOPY v.x() TO b()
PRINT "ok"
SYSTEM
