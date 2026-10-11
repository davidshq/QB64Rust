$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): DIM a() and REDIM r() in a SUB, nothing between the
' parentheses: a scalar as in the main module, or an array.
ON ERROR GOTO h
s: s

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    DIM a() AS LONG
    REDIM r() AS LONG
    a = a + 1
    r = r + 2
    PRINT "scalars:"; a; r
    PRINT "a(1):";
    PRINT a(1)
    PRINT "after DIM a(3) AS LONG:";
    DIM a(3) AS LONG
    PRINT UBOUND(a)
END SUB
