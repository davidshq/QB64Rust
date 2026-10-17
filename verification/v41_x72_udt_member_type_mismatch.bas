$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (72_udt_member_type_mismatch)
TYPE p
    x(1 TO 3) AS INTEGER
END TYPE
DIM v AS p, b(0 TO 9) AS LONG
_ARRAYCOPY v.x() TO b()
PRINT "ok"
SYSTEM
