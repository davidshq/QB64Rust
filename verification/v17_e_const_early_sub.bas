$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a SUB defined before the CONST line uses the name.
SUB early
    PRINT "early:"; c1
END SUB
CONST c1 = 5
early
PRINT "main:"; c1
SYSTEM
