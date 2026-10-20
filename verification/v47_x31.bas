$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): two handlers for button 0.
ON STRIG(0) GOSUB h
ON STRIG(0) GOSUB h2
PRINT "ok"
SYSTEM
h:
RETURN
h2:
RETURN
