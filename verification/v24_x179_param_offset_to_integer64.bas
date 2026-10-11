$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an _INTEGER64 array passed to an _OFFSET array parameter.
DIM q(3) AS _INTEGER64
fill q()
PRINT q(1)
SYSTEM
SUB fill (a() AS _OFFSET)
    a(1) = 5
END SUB
