$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a _BIT * 4 array passed to an array parameter of that type.
DIM b(3) AS _BIT * 4
s b()
PRINT b(1)
SYSTEM
SUB s (a() AS _BIT * 4)
    a(1) = 5
END SUB
