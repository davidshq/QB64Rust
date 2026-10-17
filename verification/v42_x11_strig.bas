$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (11_strig)
ON STRIG(0, 1) GOSUB h
STRIG(0, 1) ON
STRIG ON
PRINT "ok"
SYSTEM
h:
RETURN
