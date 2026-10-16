$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): does the old compiler accept this? (28_param_untyped)
DIM b(2)
S b()
PRINT b(1)
PRINT "ok"
SYSTEM
SUB S (p())
    DIM q(2)
    q(1) = 6
    p(0) = 1
    p() = q()
END SUB
