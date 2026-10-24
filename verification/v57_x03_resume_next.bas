$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RESUME NEXT in a SUB called by a main-module handler.
ON ERROR GOTO h
tries = 0
ERROR 5
PRINT "after ERROR"
SYSTEM
h:
PRINT "handler"; ERR
tries = tries + 1
IF tries > 2 THEN SYSTEM
r
PRINT "after r"
RESUME NEXT
SUB r
    PRINT "in r"
    RESUME NEXT
    PRINT "after RESUME in r"
END SUB
