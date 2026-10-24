$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RESUME of a main-module label in a SUB called by a handler.
ON ERROR GOTO h
ERROR 5
PRINT "after ERROR"
SYSTEM
back:
PRINT "at back in main"
SYSTEM
h:
PRINT "handler"; ERR
r
RESUME NEXT
SUB r
    RESUME back
END SUB
