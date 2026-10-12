$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): ERASE of one element.
DIM a(3) AS LONG
a(1) = 5: a(2) = 6
ERASE a(1)
PRINT a(1); a(2)
SYSTEM
