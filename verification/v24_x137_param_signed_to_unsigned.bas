$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a LONG array passed to an _UNSIGNED LONG array parameter.
DIM q(3) AS LONG
fill q()
PRINT q(1)
SYSTEM
SUB fill (a() AS _UNSIGNED LONG)
    a(1) = 4000000000
END SUB
