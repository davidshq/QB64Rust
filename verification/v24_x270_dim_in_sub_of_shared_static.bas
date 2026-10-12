$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): DIM in a SUB of the name and type of a DIM SHARED static array
' of the main module, with other bounds: the shared array, a local one, or an error.
ON ERROR GOTO h
DIM SHARED a(3) AS LONG
a(1) = 7
s: s
PRINT "main:"; a(1); UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    DIM a(5) AS LONG
    PRINT "SUB:"; a(1); UBOUND(a)
    a(1) = a(1) + 1
END SUB
