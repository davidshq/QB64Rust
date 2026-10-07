$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): LBOUND and UBOUND of a numeric array written with empty parentheses.
DIM x(2 TO 6) AS LONG
PRINT LBOUND(x()); UBOUND(x())
SYSTEM
