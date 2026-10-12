$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): under OPTION BASE 1, DIM with a CONST that is 0.
OPTION BASE 1
CONST zero = 0
DIM a(zero)
PRINT LBOUND(a); UBOUND(a)
SYSTEM
