$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a SUB that never indexes its array parameter, called with an
' array of one and of two dimensions.
ON ERROR GOTO h
DIM x(3) AS LONG, y(2, 4) AS LONG
show x()
show y()

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB show (m() AS LONG)
    PRINT UBOUND(m, 1);
    PRINT UBOUND(m, 2)
END SUB
