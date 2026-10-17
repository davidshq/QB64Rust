$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (30_on_timer_no_parens)
ON TIMER GOSUB h
PRINT "ok"
SYSTEM
h:
RETURN
