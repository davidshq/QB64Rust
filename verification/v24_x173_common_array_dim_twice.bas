$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON SHARED a(), DIM a(3), then DIM a(3) again.
COMMON SHARED a() AS LONG
DIM a(3) AS LONG
DIM a(3) AS LONG
PRINT UBOUND(a)
SYSTEM
