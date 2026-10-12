$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a scalar passed to an array parameter.
DIM y AS LONG
fill y
PRINT y
SYSTEM
SUB fill (a() AS LONG)
    a(1) = 5
END SUB
