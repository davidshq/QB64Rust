$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): OPTION BASE with an expression.
OPTION BASE 0 + 1
DIM a(3)
PRINT LBOUND(a)
SYSTEM
