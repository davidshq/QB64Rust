$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): CLEAR in an error handler, then RESUME NEXT.
x = 5
ON ERROR GOTO h
ERROR 5
PRINT "after"; x
SYSTEM
h:
CLEAR
RESUME NEXT
