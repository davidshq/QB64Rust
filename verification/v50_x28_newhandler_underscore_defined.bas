$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, review): does the old compiler accept this? (x28_newhandler_underscore_defined)
ON ERROR GOTO _NEWHANDLER _h
ERROR 5
PRINT "ok"
SYSTEM
_h:
PRINT "handled"; ERR
RESUME NEXT
