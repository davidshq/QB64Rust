$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, task 1.2): does the old compiler accept this? (x24_newhandler_in_sub_label)
s
PRINT "ok"
SYSTEM
SUB s
ON ERROR GOTO _NEWHANDLER h
EXIT SUB
h:
RESUME NEXT
END SUB
