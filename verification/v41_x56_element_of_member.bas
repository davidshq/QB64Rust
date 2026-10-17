$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (56_element_of_member)
TYPE p
    x(1 TO 3) AS LONG
END TYPE
DIM v(1 TO 2) AS p, b(0 TO 9) AS LONG
_ARRAYCOPY v().x() TO b()
PRINT "ok"
SYSTEM
