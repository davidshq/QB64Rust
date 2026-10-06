$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a SUB constant used in the main module after the SUB.
SUB s
    CONST pc = 9
    PRINT "in s:"; pc
END SUB
s
PRINT "main:"; pc
SYSTEM
