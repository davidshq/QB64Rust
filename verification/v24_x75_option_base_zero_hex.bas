$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): under OPTION BASE 1, DIM with &H0.
OPTION BASE 1
DIM a(&H0)
PRINT LBOUND(a); UBOUND(a)
SYSTEM
