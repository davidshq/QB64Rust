$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (55_member_udt)
TYPE q
    y AS LONG
END TYPE
TYPE p
    x(1 TO 3) AS q
END TYPE
DIM v AS p, w(1 TO 3) AS q
_ARRAYCOPY v.x() TO w()
PRINT "ok"
SYSTEM
