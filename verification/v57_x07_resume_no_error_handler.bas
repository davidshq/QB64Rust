$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RESUME NEXT in a SUB with no error pending, under a handler.
ON ERROR GOTO h
r
PRINT "after r"
SYSTEM
h:
PRINT "handler"; ERR
RESUME NEXT
SUB r
    PRINT "in r"
    RESUME NEXT
    PRINT "after RESUME in r"
END SUB
