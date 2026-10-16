$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (45_param_dims_differ_use)
DIM b(2) AS LONG
S b()
PRINT "ok"
SYSTEM
SUB S (p() AS LONG)
    DIM q(2, 2) AS LONG
    p(1) = 1
    p() = q()
END SUB
