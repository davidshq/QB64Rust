$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): ON STRIG(1) with a second handler for the same button.
ON STRIG(1) GOSUB h
ON STRIG(1) GOSUB h2
PRINT "ok"
SYSTEM
h:
RETURN
h2:
RETURN
