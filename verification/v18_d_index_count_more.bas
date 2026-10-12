$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): more indexes than dimensions.
DIM x(10) AS LONG
x(1, 2) = 5
PRINT x(1)
SYSTEM
