$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array parameter written with a comma between the parentheses,
' used with one index.
DIM x(3) AS LONG
s x()
PRINT x(1)
SYSTEM
SUB s (a(,) AS LONG)
    a(1) = 5
END SUB
