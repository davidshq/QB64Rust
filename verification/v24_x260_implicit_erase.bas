$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 5.1): ERASE of arrays used without DIM, in the main module and in a
' SUB.
ON ERROR GOTO h
z(1) = 5: s$(2) = "ab"
ERASE z, s$
PRINT "1. main:"; z(1); UBOUND(z); "["; s$(2); "]"
PRINT "2. SUB:";
p

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB p
    x(2) = 5
    ERASE x
    PRINT " after ERASE:";
    PRINT UBOUND(x)
END SUB
