$CONSOLE:ONLY
' Verification (m3-events, task 1.1): when an event is serviced. SLEEP n ended by a timer; a handler inside a
' SUB's statements; inside a FUNCTION called from an expression (between the FUNCTION's statements, before the
' caller's statement ends); no handler while an error handler runs, and what becomes of that timer after RESUME.
DIM SHARED n AS LONG
DIM t0 AS DOUBLE, i AS LONG, r AS LONG
ON ERROR GOTO oops
ON TIMER(0.5) GOSUB tick
TIMER ON
t0 = TIMER(0.001)
SLEEP 10
PRINT "1. SLEEP 10 ended after about"; INT((TIMER(0.001) - t0) * 10 + 0.5) / 10; "s, n ="; n
n = 0
busy
PRINT "2. after a SUB of 0.7 s, n ="; n
n = 0
r = f + 0 * n
PRINT "3. a FUNCTION of 0.7 s: it saw"; r; ", n ="; n
n = 0
ERROR 5
PRINT "5. after RESUME NEXT, n ="; n
_DELAY 1.2
PRINT "6. 1.2 s later, n ="; n
TIMER OFF
TIMER ON
_DELAY 0.7
PRINT "7. after TIMER OFF, TIMER ON and 0.7 s, n ="; n
TIMER OFF
SYSTEM

tick:
n = n + 1
PRINT "tick"
RETURN

oops:
PRINT "4. in the error handler for 0.7 s, n ="; n;
t0 = TIMER(0.001)
DO: LOOP WHILE TIMER(0.001) - t0 < 0.7
PRINT "->"; n
RESUME NEXT

SUB busy
    DIM t AS DOUBLE
    t = TIMER(0.001)
    DO: LOOP WHILE TIMER(0.001) - t < 0.7
    PRINT "in busy, n ="; n
END SUB

FUNCTION f&
    DIM t AS DOUBLE
    t = TIMER(0.001)
    DO: LOOP WHILE TIMER(0.001) - t < 0.7
    f& = n
END FUNCTION
