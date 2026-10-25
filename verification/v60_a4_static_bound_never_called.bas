$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): STATIC v(n) and STATIC w(f) in SUBs that are never called: the start
' code still runs f (counted in a shared variable).
DIM SHARED n AS LONG, c AS LONG
n = 3
PRINT "calls:"; c
SYSTEM

FUNCTION f
    c = c + 1
    f = 2
END FUNCTION

SUB s
    STATIC v(n) AS LONG
    PRINT UBOUND(v)
END SUB

SUB t
    STATIC w(f) AS LONG
    PRINT UBOUND(w)
END SUB
