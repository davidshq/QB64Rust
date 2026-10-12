$CONSOLE:ONLY
' Verification (m2-control-flow-slice, task 5.1): ON ERROR GOTO in a SUB naming a label that stands in the SUB and
' in main.
t
PRINT "back in main"
SYSTEM
h:
PRINT "main handler"; ERR
RESUME NEXT
SUB t
    ON ERROR GOTO h
    ERROR 5
    PRINT "after error in t"
    EXIT SUB
    h:
    PRINT "t handler"; ERR
END SUB
