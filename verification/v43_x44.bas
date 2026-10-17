$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): the error functions, one form: ERL and _ERRORLINE after an error in a GOSUB
ON ERROR GOTO h
GOSUB g
PRINT "after:"; _ERRORLINE
SYSTEM
g:
ERROR 5
RETURN
h:
PRINT "handler:"; _ERRORLINE; ERL
RESUME NEXT
PRINT "accepted"
SYSTEM
