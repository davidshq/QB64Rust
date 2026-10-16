$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): The name of a SUB with a parameter and a colon, no jump to it: a label (it cannot be a call).
one:
PRINT "main"
CALL one(3)
SYSTEM
SUB one (k AS INTEGER)
    PRINT "sub one"; k
END SUB
