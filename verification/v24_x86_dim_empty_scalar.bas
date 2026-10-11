$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): what DIM a() AS LONG declares: a stored and read as a scalar.
DIM a() AS LONG
a = 100000.5
PRINT a; LEN(a)
SYSTEM
