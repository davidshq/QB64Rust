$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 13, RESUME 0 in a FUNCTION called by a main-module handler:
' where the program goes on, and the value the call gives.
ON ERROR GOTO h
tries = 0
ERROR 5
PRINT "after ERROR"
SYSTEM
h:
PRINT "handler"; ERR
tries = tries + 1
IF tries > 2 THEN SYSTEM
v = r(3)
PRINT "after r:"; v
RESUME NEXT
FUNCTION r (k)
    PRINT "in r"
    r = k
    RESUME 0
    PRINT "after RESUME 0 in r"
END FUNCTION
