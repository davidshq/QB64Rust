$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array of one user type passed to an array parameter of
' another.
TYPE r1
    n AS LONG
END TYPE
TYPE r2
    n AS LONG
END TYPE
DIM q(3) AS r1
fill q()
PRINT q(1).n
SYSTEM
SUB fill (a() AS r2)
    a(1).n = 5
END SUB
