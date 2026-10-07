$CONSOLE:ONLY
' Verification (m2-control-flow-slice, task 5.1): RESUME label inside a SUB, naming a label of the SUB.
ON ERROR GOTO h
t
PRINT "back in main"
SYSTEM
h:
PRINT "handler"; ERR
RESUME NEXT
SUB t
    ERROR 5
    back:
    PRINT "at back"
    EXIT SUB
    RESUME back
END SUB
