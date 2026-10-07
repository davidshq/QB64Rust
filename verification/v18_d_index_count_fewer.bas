$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): fewer indexes than dimensions.
DIM y(3, 3) AS LONG
y(1) = 5
PRINT y(1, 0)
SYSTEM
