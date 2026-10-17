$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (32_on_timer_gosub_alone)
ON TIMER(1) GOSUB
PRINT "ok"
SYSTEM
h:
RETURN
