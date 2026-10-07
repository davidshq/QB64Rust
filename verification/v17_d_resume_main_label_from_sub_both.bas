$CONSOLE:ONLY
' Verification (m2-control-flow-slice, task 5.1): RESUME label inside a SUB, naming a label that stands in the SUB
' and in main.
ON ERROR GOTO h
t
PRINT "back in main"
SYSTEM
back:
PRINT "main back"
SYSTEM
h:
PRINT "handler"; ERR
RESUME NEXT
SUB t
    ERROR 5
    back:
    PRINT "t back"
    EXIT SUB
    RESUME back
END SUB
