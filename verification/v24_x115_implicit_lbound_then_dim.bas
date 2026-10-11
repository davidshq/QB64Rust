$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): LBOUND(a) of a name that is no array, then DIM a(5).
PRINT LBOUND(a)
DIM a(5)
PRINT UBOUND(a)
SYSTEM
