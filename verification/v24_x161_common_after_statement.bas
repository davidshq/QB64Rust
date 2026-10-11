$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON below an executable statement.
x = 1
COMMON y AS LONG
y = 2: PRINT x; y
SYSTEM
