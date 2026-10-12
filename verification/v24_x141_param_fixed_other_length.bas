$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a STRING * 3 array passed to a STRING * 4 array parameter.
DIM q(3) AS STRING * 3
fill q()
PRINT q(1)
SYSTEM
SUB fill (a() AS STRING * 4)
    a(1) = "abcdef"
END SUB
