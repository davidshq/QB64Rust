$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): an error in SUB t under a main handler that calls SUB r, whose RESUME NEXT runs; at most three passes.
ON ERROR GOTO h
t
PRINT "after t"
SYSTEM
h:
PRINT "handler"; ERR
tries = tries + 1
IF tries > 2 THEN SYSTEM
r
PRINT "after r"
RESUME NEXT
SUB t
    PRINT "in t"
    ERROR 6
    PRINT "after ERROR in t"
END SUB
SUB r
    RESUME NEXT
END SUB
