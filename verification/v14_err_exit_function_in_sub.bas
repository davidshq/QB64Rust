$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): EXIT FUNCTION inside a SUB.
SUB s (a AS LONG)
PRINT a
EXIT FUNCTION
PRINT "not here"
END SUB
s 1
SYSTEM
