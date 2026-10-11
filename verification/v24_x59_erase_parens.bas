$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): ERASE with empty parentheses after the name.
DIM a(3) AS LONG
ERASE a()
PRINT 1
SYSTEM
