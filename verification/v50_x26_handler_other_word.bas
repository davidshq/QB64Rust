$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, task 1.2): does the old compiler accept this? (x26_handler_other_word)
ON ERROR GOTO foo h
PRINT "ok"
SYSTEM
h:
RESUME NEXT
