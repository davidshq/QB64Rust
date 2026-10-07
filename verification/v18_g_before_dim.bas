$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): an element used before the array's DIM line.
a(1) = 5
DIM a(5) AS SINGLE
PRINT a(1)
SYSTEM
