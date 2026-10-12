$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7, review): a second opening parenthesis between an array parameter's parentheses.
DIM x(3) AS LONG
one x()
SYSTEM
SUB one (a(b(2)) AS LONG)
    PRINT a(1)
END SUB
