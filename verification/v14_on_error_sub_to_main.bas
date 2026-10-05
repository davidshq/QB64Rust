$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): ON ERROR GOTO inside a SUB naming a label of the main module.
SUB t
    ON ERROR GOTO mainh
    ERROR 5
    PRINT "after error in t"
END SUB
t
PRINT "back in main"
SYSTEM
mainh:
PRINT "main handler, err"; ERR
RESUME NEXT
