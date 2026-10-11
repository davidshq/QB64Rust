$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): STATIC a(5) in a SUB where the main module has DIM SHARED a(3)
' of that type.
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
    STATIC a(5) AS LONG
    PRINT "SUB:"; a(1); UBOUND(a)
    a(1) = a(1) + 1
END SUB
