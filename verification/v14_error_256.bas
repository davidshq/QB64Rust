$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): ERROR 256 with a handler active.
ON ERROR GOTO handler
PRINT "e256": ERROR 256
PRINT "done"
SYSTEM
handler:
PRINT "err"; ERR
RESUME NEXT
