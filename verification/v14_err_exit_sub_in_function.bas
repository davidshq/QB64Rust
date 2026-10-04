$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): EXIT SUB inside a FUNCTION.
FUNCTION f& (a AS LONG)
f& = a
EXIT SUB
END FUNCTION
PRINT f&(1)
SYSTEM
