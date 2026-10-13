$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): The name of a SUB without parameters and a colon, twice in one body, and no jump to it: two calls.
one:
one:
PRINT "main"
SYSTEM
SUB one
    PRINT "sub one"
END SUB
