$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): ON STRIG with three operands.
ON STRIG(1, 2, 3) GOSUB h
PRINT "ok"
SYSTEM
h:
RETURN
