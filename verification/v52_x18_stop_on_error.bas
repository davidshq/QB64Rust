$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x18_stop_on_error)
' STOP inside an error handler.
ON ERROR GOTO h
ERROR 5
PRINT "not"
SYSTEM
h:
PRINT "h"; ERR
STOP
