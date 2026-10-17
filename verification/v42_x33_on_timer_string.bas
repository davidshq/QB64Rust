$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (33_on_timer_string)
ON TIMER("a") GOSUB h
PRINT "ok"
SYSTEM
h:
RETURN
