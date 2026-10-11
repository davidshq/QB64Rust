$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array in parentheses passed to an array parameter.
DIM x(3) AS LONG
fill (x())
PRINT x(1)
SYSTEM
SUB fill (a() AS LONG)
    a(1) = 5
END SUB
