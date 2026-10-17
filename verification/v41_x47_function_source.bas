$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (47_function_source)
DIM b(0 TO 9) AS LONG
_ARRAYCOPY fn() TO b()
PRINT "ok"
SYSTEM

FUNCTION fn&
    fn& = 1
END FUNCTION
