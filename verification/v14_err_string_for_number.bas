$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): a string argument for a LONG parameter.
SUB s (a AS LONG)
PRINT a
END SUB
CALL s("x")
SYSTEM
