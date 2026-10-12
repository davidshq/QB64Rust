$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a SUB defined before the CONST line, not using the name.
SUB early
    PRINT "early"
END SUB
CONST c1 = 5
early
PRINT "main:"; c1
SYSTEM
