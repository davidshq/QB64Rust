$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): a DECLARE that disagrees with the definition, with a call that matches the definition.
DECLARE SUB s (a AS LONG)
s "x"
SUB s (a AS STRING)
PRINT a
END SUB
SYSTEM
