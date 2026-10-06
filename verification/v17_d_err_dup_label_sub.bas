$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): the same label twice in one SUB.
t
SYSTEM
SUB t
    a:
    PRINT "x"
    a:
    PRINT "y"
END SUB
