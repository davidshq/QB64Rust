$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): The name of a SUB and a colon inside that SUB, no jump to it: a call of itself.
DIM SHARED n AS INTEGER
CALL one
PRINT "main"; n
SYSTEM
SUB one
    n = n + 1
    IF n >= 3 THEN EXIT SUB
    one:
END SUB
