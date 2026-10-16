$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): The same with a jump to the name: the two lines are labels, and the second is a duplicate.
DIM n AS INTEGER
one:
one:
n = n + 1
IF n < 2 THEN GOTO one
PRINT "main"
SYSTEM
SUB one
    PRINT "sub one"
END SUB
