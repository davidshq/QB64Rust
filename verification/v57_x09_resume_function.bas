$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RESUME NEXT in a FUNCTION called by a handler.
ON ERROR GOTO h
ERROR 5
PRINT "after ERROR"
SYSTEM
h:
PRINT "handler"; ERR
x = f
PRINT "after f"
RESUME NEXT
FUNCTION f
    RESUME NEXT
END FUNCTION
