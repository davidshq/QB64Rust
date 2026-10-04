$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): a SUB called without CALL but with two arguments in parentheses.
SUB s (a AS LONG, b AS LONG)
PRINT a; b
END SUB
s (1, 2)
SYSTEM
