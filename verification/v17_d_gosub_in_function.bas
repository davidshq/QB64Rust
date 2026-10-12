$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): GOSUB inside a FUNCTION called from a PRINT.
PRINT "a"; f; "b"
SYSTEM
FUNCTION f
    GOSUB l
    EXIT FUNCTION
    l:
    f = 5
    RETURN
END FUNCTION
