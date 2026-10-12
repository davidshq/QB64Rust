$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, task 4.1): DIM with another number of dimensions than the dynamic array
' has.
n = 2
REDIM m(2, 3) AS LONG
DIM m(n) AS LONG
PRINT UBOUND(m)
SYSTEM
