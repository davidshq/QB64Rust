$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, task 1.2): does the old compiler accept this? (x04_newhandler_zero)
ON ERROR GOTO _NEWHANDLER 0
PRINT "ok"
SYSTEM
