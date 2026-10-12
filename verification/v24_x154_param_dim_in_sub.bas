$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM of the array parameter's name inside the SUB.
ON ERROR GOTO h
DIM x(3) AS LONG
s x()
PRINT UBOUND(x)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s (a() AS LONG)
    DIM a(5) AS LONG
    PRINT UBOUND(a);
END SUB
