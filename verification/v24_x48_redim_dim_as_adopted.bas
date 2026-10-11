$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM x(n) AS LONG with a variable bound, then REDIM x(7) without
' AS.
n = 3
DIM x(n) AS LONG
REDIM x(7)
x(1) = 100000: PRINT UBOUND(x); x(1)
SYSTEM
