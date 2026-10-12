$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): an array name without indexes in an expression.
DIM x(10) AS LONG
PRINT x
x = 3
PRINT x; x(0)
SYSTEM
