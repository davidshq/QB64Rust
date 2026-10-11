$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an _UNSIGNED LONG array passed to a LONG array parameter.
DIM q(3) AS _UNSIGNED LONG
fill q()
PRINT q(1)
SYSTEM
SUB fill (a() AS LONG)
    a(1) = -1
END SUB
