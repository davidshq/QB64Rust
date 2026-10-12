$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON SHARED a(), then DIM a(n) with a variable bound, seen in
' a SUB.
ON ERROR GOTO h
COMMON SHARED a() AS LONG
n = 3
DIM a(n) AS LONG
a(2) = 6
s
REDIM a(5) AS LONG
PRINT UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    PRINT a(2); UBOUND(a);
END SUB
