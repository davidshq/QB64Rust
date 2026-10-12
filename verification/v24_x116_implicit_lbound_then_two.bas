$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): LBOUND(a) of a name that is no array, then a(1, 2).
PRINT LBOUND(a)
a(1, 2) = 3
PRINT UBOUND(a, 2)
SYSTEM
