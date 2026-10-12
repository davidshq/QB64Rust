$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): in a SUB ... STATIC, DIM with constant bounds of the name and
' type of a DIM SHARED static array of the main module.
ON ERROR GOTO h
DIM SHARED a(3) AS LONG
a(1) = 7
s: s
PRINT "main:"; a(1); UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s STATIC
    DIM a(5) AS LONG
    PRINT "SUB:"; a(1); UBOUND(a)
    a(1) = a(1) + 1
END SUB
