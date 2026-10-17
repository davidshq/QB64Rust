$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (27_on_timer_empty)
ON TIMER() GOSUB h
PRINT "ok"
SYSTEM
h:
RETURN
