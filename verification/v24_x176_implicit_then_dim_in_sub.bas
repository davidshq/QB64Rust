$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): in a SUB, an array used without DIM, then DIM of it in the same
' SUB, called twice.
ON ERROR GOTO h
s: s

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    PRINT "before DIM:";
    a(1) = 5
    DIM a(3)
    a(1) = a(1) + 1
    PRINT a(1); UBOUND(a)
END SUB
