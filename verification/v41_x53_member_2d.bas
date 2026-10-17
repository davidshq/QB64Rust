$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (53_member_2d)
TYPE p
    x(1 TO 3, 1 TO 2) AS LONG
END TYPE
DIM v AS p, n(0 TO 3, 0 TO 4) AS LONG
_ARRAYCOPY v.x(1 TO 2, 1 TO 2) TO n(0, 0)
PRINT "ok"
SYSTEM
