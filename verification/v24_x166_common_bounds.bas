$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON a(3) with bounds.
COMMON a(3) AS LONG
a(2) = 1: PRINT UBOUND(a)
SYSTEM
