$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM of a name that is a scalar variable.
DIM x AS LONG
x = 5
REDIM x(3) AS LONG
x(1) = 2: PRINT x; x(1)
SYSTEM
