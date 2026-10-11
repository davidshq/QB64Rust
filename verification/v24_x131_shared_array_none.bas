$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): SHARED n() in a SUB when the main module has no such array.
ON ERROR GOTO h
s
PRINT "main: n(1), UBOUND(n)";
PRINT n(1); UBOUND(n)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    SHARED n() AS LONG
    PRINT "SUB: n(1) = 5:";
    n(1) = 5
    PRINT n(1); UBOUND(n)
END SUB
