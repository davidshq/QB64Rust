$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): under OPTION BASE 1, DIM with 00.
OPTION BASE 1
DIM a(00)
PRINT LBOUND(a); UBOUND(a)
SYSTEM
