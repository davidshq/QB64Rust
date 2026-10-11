$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): OPTION BASE with a CONST.
CONST one = 1
OPTION BASE one
DIM a(3)
PRINT LBOUND(a)
SYSTEM
