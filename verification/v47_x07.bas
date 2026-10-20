$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): ON STRIG with no first operand.
ON STRIG(, 1) GOSUB h
PRINT "ok"
SYSTEM
h:
RETURN
