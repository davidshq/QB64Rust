$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (17_param_unknown_dims)
DIM b(2) AS LONG
S b()
PRINT "ok"
SYSTEM
SUB S (p() AS LONG)
    DIM q(2) AS LONG
    q(1) = 6
    p() = q()
    PRINT p(1)
END SUB
