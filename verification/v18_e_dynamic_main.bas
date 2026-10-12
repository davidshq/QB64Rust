$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a non-constant bound in the main module (a dynamic array).
n = 5
DIM a(n) AS LONG
a(5) = 1
PRINT LBOUND(a); UBOUND(a); a(5)
SYSTEM
