$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): ON ERROR GOTO with a label inside a SUB.
SUB t
    ON ERROR GOTO h
    ERROR 5
    PRINT "after error in t"
    EXIT SUB
    h:
    PRINT "handler in t, err"; ERR
    RESUME NEXT
END SUB
t
PRINT "back in main"
SYSTEM
