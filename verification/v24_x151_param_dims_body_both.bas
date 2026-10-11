$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a SUB that uses its array parameter with one index and with two.
DIM x(3) AS LONG
both x()
SYSTEM
SUB both (m() AS LONG)
    m(1) = 5
    m(1, 2) = 5
END SUB
