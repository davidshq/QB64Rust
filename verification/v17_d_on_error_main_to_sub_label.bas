$CONSOLE:ONLY
' Verification (m2-control-flow-slice, task 5.1): ON ERROR GOTO in main naming a label that stands only in a SUB.
ON ERROR GOTO h
ERROR 5
PRINT "after"
SYSTEM
SUB t
    h:
    PRINT "in t"
END SUB
