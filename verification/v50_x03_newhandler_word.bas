$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, task 1.2): does the old compiler accept this? (x03_newhandler_word)
ON ERROR GOTO _NEWHANDLER x h
PRINT "ok"
SYSTEM
h:
RESUME NEXT
