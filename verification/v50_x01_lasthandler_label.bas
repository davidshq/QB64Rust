$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, task 1.2): does the old compiler accept this? (x01_lasthandler_label)
ON ERROR GOTO h
ON ERROR GOTO _LASTHANDLER h
PRINT "ok"
SYSTEM
h:
RESUME NEXT
