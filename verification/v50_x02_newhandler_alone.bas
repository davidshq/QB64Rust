$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, task 1.2): does the old compiler accept this? (x02_newhandler_alone)
ON ERROR GOTO _NEWHANDLER
PRINT "ok"
SYSTEM
h:
RESUME NEXT
