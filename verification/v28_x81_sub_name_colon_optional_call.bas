$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): A jump in one SUB makes the name a label in every body: the main module's line is no call any more.
one:
PRINT "main"
CALL two
SYSTEM
SUB one
    PRINT "sub one"
END SUB
SUB two
    DIM n AS INTEGER
    one:
    n = n + 1
    IF n < 2 THEN GOTO one
    PRINT "two"; n
END SUB
