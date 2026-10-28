$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 11, _STATUSCODE of a host handle opened and closed (its entry
' gone from libqb's list), and of a host handle still open (another kind of handle).
ON ERROR GOTO h
DIM hst AS LONG
hst = _OPENHOST("TCP/IP:47319")
PRINT "opened:"; hst < 0
PRINT "open host:"; _STATUSCODE(hst)
CLOSE hst
PRINT "closed host:"; _STATUSCODE(hst)
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
