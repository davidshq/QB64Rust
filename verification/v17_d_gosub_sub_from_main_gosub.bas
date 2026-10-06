$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): main GOSUB, the subroutine calls a SUB that does its own GOSUB and RETURN.
GOSUB g1
PRINT "back in main"
SYSTEM
g1:
PRINT "in g1"
sg
PRINT "g1 after sg"
RETURN
SUB sg
    GOSUB l
    PRINT "sg done"
    EXIT SUB
    l:
    PRINT "in sg l"
    RETURN
END SUB
