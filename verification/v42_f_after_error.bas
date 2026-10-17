$CONSOLE:ONLY
' Verification (m3-events, task 1.1): a timer event that comes while an error handler runs. Is it serviced after
' RESUME NEXT, while plain statements run (a busy loop, a SUB's loop), and does a _DELAY, SLEEP or TIMER ON bring
' it back?
DIM SHARED n AS LONG
DIM t0 AS DOUBLE
ON ERROR GOTO oops
ON TIMER(0.3) GOSUB tick
TIMER ON
ERROR 5
t0 = TIMER(0.001)
DO: LOOP WHILE TIMER(0.001) - t0 < 1
PRINT "2. a loop of 1 s after RESUME NEXT, n ="; n
busy
PRINT "3. a SUB's loop of 1 s, n ="; n
TIMER ON
PRINT "4. right after TIMER ON again, n ="; n
TIMER OFF
n = 0
TIMER ON
ERROR 5
_DELAY 0.05
PRINT "5. after the second error, _DELAY 0.05: n ="; n
TIMER OFF
SYSTEM

tick:
n = n + 1
RETURN

oops:
t0 = TIMER(0.001)
DO: LOOP WHILE TIMER(0.001) - t0 < 0.4
PRINT "1. after 0.4 s in the error handler, n ="; n
RESUME NEXT

SUB busy
    DIM t AS DOUBLE
    t = TIMER(0.001)
    DO: LOOP WHILE TIMER(0.001) - t < 1
END SUB
