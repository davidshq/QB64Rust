$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): GOTO from main to a label that stands only in a SUB.
GOTO inside
SYSTEM
SUB t
    inside:
    PRINT "in t"
END SUB
