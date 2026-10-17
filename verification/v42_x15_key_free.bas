$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (15_key_free)
ON KEY(1) GOSUB h
KEY(1) FREE
PRINT "ok"
SYSTEM
h:
RETURN
