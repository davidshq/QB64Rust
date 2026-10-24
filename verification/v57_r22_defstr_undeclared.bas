$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): RUN of a name never assigned under DEFSTR, under a handler.
DEFSTR A-Z
ON ERROR GOTO h
RUN p
PRINT "after RUN"
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
