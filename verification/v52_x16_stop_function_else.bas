$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x16_stop_function_else)
' STOP in a FUNCTION after ELSE.
PRINT f(0)
PRINT "after"
SYSTEM
FUNCTION f (n)
    IF n THEN f = 1 ELSE PRINT "s": STOP
END FUNCTION
