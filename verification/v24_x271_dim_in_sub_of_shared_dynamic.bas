$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): DIM in a SUB of the name and type of a REDIM SHARED array of
' the main module: the shared array (error 10), or a local one.
ON ERROR GOTO h
REDIM SHARED d(3) AS LONG
d(1) = 7
s: s
PRINT "main:"; d(1); UBOUND(d)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    DIM d(5) AS LONG
    PRINT "SUB:"; d(1); UBOUND(d)
    d(1) = d(1) + 1
END SUB
