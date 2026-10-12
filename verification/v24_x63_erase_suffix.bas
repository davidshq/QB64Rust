$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): ERASE with a suffix that names the array, and one that names no
' array.
DIM a(3) AS LONG
a(1) = 5
ERASE a&
PRINT a(1)
ERASE a%
PRINT a(1); UBOUND(a%)
SYSTEM
