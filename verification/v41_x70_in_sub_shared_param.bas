$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (70_in_sub_shared_param)
DIM a(1 TO 5) AS LONG
DIM SHARED g(0 TO 9) AS LONG
fill a()
PRINT g(1)
PRINT "ok"
SYSTEM

SUB fill (x() AS LONG)
    _ARRAYCOPY x() TO g()
END SUB
