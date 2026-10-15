$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (27_param_known_dims)
DIM b(2) AS LONG
S b()
PRINT b(1)
PRINT "ok"
SYSTEM
SUB S (p() AS LONG)
    DIM q(2) AS LONG
    q(1) = 6
    p(0) = 1
    p() = q()
END SUB
