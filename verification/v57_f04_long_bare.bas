$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a program FUNCTION g& as a statement: g& 1.
g& 1
PRINT "ok"
SYSTEM
FUNCTION g& (a)
    PRINT "in g"
    g& = a
END FUNCTION
