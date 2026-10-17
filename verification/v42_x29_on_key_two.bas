$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (29_on_key_two)
ON KEY(1, 2) GOSUB h
PRINT "ok"
SYSTEM
h:
RETURN
