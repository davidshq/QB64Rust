$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (07_numbered)
t = _FREETIMER
ON TIMER(t, 0.5) GOSUB h
TIMER(t) ON
TIMER(t) STOP
TIMER(t) OFF
TIMER(t) FREE
PRINT "ok"
SYSTEM
h:
RETURN
