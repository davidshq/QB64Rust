$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): in a SUB, an array used without DIM, then STATIC of it.
ON ERROR GOTO h
s: s

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    a(1) = 5
    STATIC a(3)
    a(1) = a(1) + 1
    PRINT a(1); UBOUND(a)
END SUB
