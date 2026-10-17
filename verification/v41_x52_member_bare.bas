$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (52_member_bare)
TYPE p
    x(1 TO 3) AS LONG
END TYPE
DIM v AS p, b(0 TO 9) AS LONG
_ARRAYCOPY v.x TO b()
PRINT "ok"
SYSTEM
