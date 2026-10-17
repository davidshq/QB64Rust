$CONSOLE:ONLY
' Verification (m3-arraycopy, while its proposal was written): does the old compiler accept this? (75: array
' parameters whose number of dimensions only the _ARRAYCOPY gives, one SUB called with arrays of one dimension)
DIM a(1 TO 3) AS LONG, b(1 TO 5) AS LONG
a(1) = 1: a(2) = 2: a(3) = 3
cp a(), b()
PRINT b(1); b(2); b(3); b(4); b(5)
PRINT "ok"
SYSTEM

SUB cp (x() AS LONG, y() AS LONG)
    _ARRAYCOPY x() TO y(2)
END SUB

SUB never (x() AS LONG, y() AS LONG)
    _ARRAYCOPY x(1 TO 2, 1 TO 2) TO y(1, 1)
END SUB
