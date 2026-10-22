$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x12_stop_function_arg)
' STOP with an expression that calls a FUNCTION: is it evaluated?
PRINT "a"
STOP f(1)
PRINT "b"
SYSTEM
FUNCTION f (n)
    PRINT "f called"
    f = n
END FUNCTION
