$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (19_on_uevent)
ON UEVENT GOSUB h
PRINT "ok"
SYSTEM
h:
RETURN
