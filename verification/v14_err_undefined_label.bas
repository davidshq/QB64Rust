$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): ON ERROR GOTO a label that does not exist.
ON ERROR GOTO nowhere
PRINT "a"
SYSTEM
