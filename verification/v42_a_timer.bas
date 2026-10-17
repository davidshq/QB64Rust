$CONSOLE:ONLY
' Verification (m3-events, task 1.1): timer 0 with a GOSUB handler. Off until TIMER ON; how often it fires
' during one long statement (_DELAY) and between short ones; TIMER STOP holds one event until TIMER ON, which
' runs it at once; TIMER OFF forgets; ON TIMER again while on; the handler is not entered again while it runs.
DIM SHARED n AS LONG
DIM i AS LONG, t0 AS DOUBLE
ON TIMER(0.4) GOSUB tick
_DELAY 0.6
PRINT "1. before TIMER ON:"; n
TIMER ON
_DELAY 1
PRINT "2. one _DELAY 1 at 0.4 s:"; n
n = 0
t0 = TIMER(0.001)
DO: i = i + 1: LOOP WHILE TIMER(0.001) - t0 < 1
PRINT "3. a loop for 1 s:"; n
TIMER STOP
n = 0
_DELAY 1
PRINT "4. stopped for 1 s:"; n
TIMER ON
PRINT "5. right after TIMER ON:"; n
TIMER OFF
n = 0
_DELAY 1
PRINT "6. off for 1 s:"; n
TIMER ON
_DELAY 0.2
PRINT "7. 0.2 s after ON again:"; n
_DELAY 0.4
PRINT "8. 0.6 s after ON again:"; n
TIMER OFF
n = 0
ON TIMER(0.4) GOSUB slow
TIMER ON
_DELAY 1.3
TIMER OFF
PRINT "9. a handler of 0.5 s every 0.4 s for 1.3 s:"; n
SYSTEM

tick:
n = n + 1
RETURN

slow:
n = n + 1
_DELAY 0.5
RETURN
