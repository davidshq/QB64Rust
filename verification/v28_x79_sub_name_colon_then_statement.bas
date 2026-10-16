$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): The name of a SUB, a colon and a statement on the line: the call, then the statement.
one: PRINT "after"
PRINT "main"
SYSTEM
SUB one
    PRINT "sub one"
END SUB
