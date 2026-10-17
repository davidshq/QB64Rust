$CONSOLE:ONLY
' Verification (m3-events, task 1.1): numbered timers. _FREETIMER, two timers at once, TIMER(n) FREE and the
' reuse of the number, and the run-time errors: a timer never allocated, TIMER FREE, a released one; the number
' as a float.
DIM SHARED a AS LONG, b AS LONG
DIM t AS LONG, u AS LONG, v AS LONG
ON ERROR GOTO oops
t = _FREETIMER
PRINT "1. first _FREETIMER:"; t
u = _FREETIMER
PRINT "2. second:"; u
ON TIMER(t, 0.4) GOSUB ta
ON TIMER(u, 0.7) GOSUB tb
TIMER(t) ON
TIMER(u) ON
_DELAY 1
TIMER(t) OFF
TIMER(u) OFF
PRINT "3. a, b after 1 s:"; a; b
TIMER(t) FREE
v = _FREETIMER
PRINT "4. _FREETIMER after FREE of the first:"; v
TIMER(v) FREE
PRINT "5. ON TIMER of a released timer:";: ON TIMER(t, 1) GOSUB ta: PRINT " done"
PRINT "6. TIMER ON of a released timer:";: TIMER(t) ON: PRINT " done"
PRINT "7. ON TIMER(9, 1):";: ON TIMER(9, 1) GOSUB ta: PRINT " done"
PRINT "8. TIMER FREE:";: TIMER FREE: PRINT " done"
PRINT "9. TIMER(-1) ON:";: TIMER(-1) ON: PRINT " done"
PRINT "10. ON TIMER(u + 0.4, 1), a float rounded:";: ON TIMER(u + 0.4, 1) GOSUB ta: PRINT " done"
PRINT "11. ON TIMER(u + 0.6, 1):";: ON TIMER(u + 0.6, 1) GOSUB ta: PRINT " done"
PRINT "12. TIMER(u + 0.6) ON:";: TIMER(u + 0.6) ON: PRINT " done"
TIMER(u) OFF
PRINT "13. ON TIMER(0) of seconds 0:";: ON TIMER(0) GOSUB ta: PRINT " done"
PRINT "14. ON TIMER(-1) of seconds -1:";: ON TIMER(-1) GOSUB ta: PRINT " done"
SYSTEM

ta:
a = a + 1
RETURN

tb:
b = b + 1
RETURN

oops:
PRINT " error"; ERR;
RESUME NEXT
