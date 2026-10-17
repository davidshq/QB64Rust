$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (54_member_2d_whole)
TYPE p
    x(1 TO 3, 1 TO 2) AS LONG
END TYPE
DIM v AS p, w(1 TO 3, 1 TO 2) AS LONG
_ARRAYCOPY v.x() TO w()
PRINT "ok"
SYSTEM
