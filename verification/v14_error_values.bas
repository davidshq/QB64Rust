$CONSOLE:ONLY
' Verification (m2-procedures-and-errors, task 2.1): ERROR with 0, -1, 255, 2.5, 2.7, 3.5 and 70000; the handler prints ERR and ERL and resumes next.
ON ERROR GOTO handler
PRINT "e0": ERROR 0
PRINT "em1": ERROR -1
PRINT "e255": ERROR 255
PRINT "e2.5": ERROR 2.5
PRINT "e2.7": ERROR 2.7
PRINT "e3.5": ERROR 3.5
PRINT "e70000": ERROR 70000
PRINT "done"
SYSTEM
handler:
PRINT "err"; ERR; "erl"; ERL
RESUME NEXT
