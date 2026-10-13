$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): The name of a SUB and a colon with no DECLARE before it: still a call (the SUB stands below).
one:
PRINT "main"
SYSTEM
SUB one
    PRINT "sub one"
END SUB
