$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (71_param_untyped_dims)
DIM a(1 TO 2, 1 TO 2) AS LONG, b(1 TO 2, 1 TO 2) AS LONG
fill a(), b()
PRINT "called"
PRINT "ok"
SYSTEM

SUB fill (x() AS LONG, y() AS LONG)
    _ARRAYCOPY x() TO y(1)
END SUB
