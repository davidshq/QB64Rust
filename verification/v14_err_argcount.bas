$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): a SUB called with too many arguments.
SUB s (a AS LONG)
PRINT a
END SUB
CALL s(1, 2)
SYSTEM
