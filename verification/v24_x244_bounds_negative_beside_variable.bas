$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, task 4.1): a dimension with a single negative constant bound beside a
' dimension with a variable bound.
n = 1
DIM half(n TO 2, -1) AS LONG
PRINT UBOUND(half, 2)
SYSTEM
