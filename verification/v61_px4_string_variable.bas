$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a string variable passed to a parameter whose SADD is taken.
g$ = "abc"
t g$
PRINT g$
SYSTEM

SUB t (s$)
    PRINT SADD(s$)
END SUB
