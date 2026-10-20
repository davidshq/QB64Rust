$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): ON STRIG with a string operand.
ON STRIG("a") GOSUB h
PRINT "ok"
SYSTEM
h:
RETURN
