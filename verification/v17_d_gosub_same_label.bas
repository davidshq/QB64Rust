$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): GOSUB to a label name that exists in main and in a SUB, from both bodies.
GOSUB same
sx
PRINT "end"
SYSTEM
same:
PRINT "main same"
RETURN
SUB sx
    GOSUB same
    PRINT "sx done"
    EXIT SUB
    same:
    PRINT "sx same"
    RETURN
END SUB
