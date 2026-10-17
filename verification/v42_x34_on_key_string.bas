$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (34_on_key_string)
ON KEY("a") GOSUB h
PRINT "ok"
SYSTEM
h:
RETURN
