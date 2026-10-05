$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): a FUNCTION called with too few arguments.
FUNCTION f& (a AS LONG, b AS LONG)
f& = a + b
END FUNCTION
PRINT f&(1)
SYSTEM
