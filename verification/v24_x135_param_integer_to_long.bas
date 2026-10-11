$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an INTEGER array passed to a LONG array parameter.
DIM q(3) AS INTEGER
fill q()
PRINT q(1)
SYSTEM
SUB fill (a() AS LONG)
    a(1) = 5
END SUB
