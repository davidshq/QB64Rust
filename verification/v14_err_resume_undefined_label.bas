$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): RESUME to a label that does not exist.
ON ERROR GOTO h
ERROR 5
SYSTEM
h:
RESUME nowhere
