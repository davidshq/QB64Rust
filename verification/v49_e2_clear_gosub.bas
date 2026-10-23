$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): RETURN after CLEAR in a GOSUB, under a handler set after the CLEAR.
GOSUB g
PRINT "after gosub"
SYSTEM
g:
CLEAR
ON ERROR GOTO h
RETURN
PRINT "after return"
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
