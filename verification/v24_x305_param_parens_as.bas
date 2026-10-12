$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7, review): the word AS between an array parameter's parentheses.
DIM x(3) AS LONG
one x()
SYSTEM
SUB one (a(1 AS LONG))
    PRINT a(1)
END SUB
