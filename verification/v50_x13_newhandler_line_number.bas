$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, task 1.2): does the old compiler accept this? (x13_newhandler_line_number)
ON ERROR GOTO _NEWHANDLER 10
ERROR 5
PRINT "ok"
SYSTEM
10 PRINT "h"; ERR
RESUME NEXT
