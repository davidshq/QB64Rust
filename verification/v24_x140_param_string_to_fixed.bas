$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a STRING array passed to a STRING * 3 array parameter.
DIM q(3) AS STRING
fill q()
PRINT q(1)
SYSTEM
SUB fill (a() AS STRING * 3)
    a(1) = "abcdef"
END SUB
