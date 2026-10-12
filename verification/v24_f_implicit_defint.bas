$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array used without DIM under DEFINT A-Z and under OPTION BASE
' 1.
DEFINT A-Z
OPTION BASE 1
k(1) = 2.5
PRINT k(1); LEN(k(1)); LBOUND(k); UBOUND(k); UBOUND(k%)
SYSTEM
