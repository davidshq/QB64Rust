$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an element of a _BIT array passed to a SUB.
DIM b4(3) AS _BIT * 4
b4(0) = 3: b4(1) = 5
show b4(1)
PRINT b4(1)
SYSTEM
SUB show (v AS LONG)
    PRINT v
    v = 0
END SUB
