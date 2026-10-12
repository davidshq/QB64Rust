$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of an array parameter with another number of dimensions
' than the argument has.
ON ERROR GOTO h
REDIM x(3) AS LONG
s x()
PRINT UBOUND(x)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s (a() AS LONG)
    REDIM a(2, 2) AS LONG
END SUB
