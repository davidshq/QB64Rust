$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, task 4.1): a dimension with constant bounds in the wrong order beside a
' dimension with a variable bound.
n = 1
DIM part(n, 3 TO 1) AS LONG
PRINT UBOUND(part, 2)
SYSTEM
