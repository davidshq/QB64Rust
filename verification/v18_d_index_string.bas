$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a string index is a compile error.
DIM x(10) AS LONG
x("a") = 1
PRINT x(1)
SYSTEM
