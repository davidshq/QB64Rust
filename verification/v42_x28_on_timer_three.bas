$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (28_on_timer_three)
ON TIMER(1, 2, 3) GOSUB h
PRINT "ok"
SYSTEM
h:
RETURN
