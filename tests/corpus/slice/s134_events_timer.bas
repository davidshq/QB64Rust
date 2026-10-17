$CONSOLE:ONLY
' Slice (m3-events, task 3.3): event trapping in a console program, recorded with qb64pe.exe. Timers at 0.6 s or
' more, each count read at least 0.3 s away from an event, so that load does not change the output: off until
' TIMER ON, STOP holding one event until ON, OFF forgetting it; SUB handlers with their argument; a handler set
' up in a SUB; numbered timers, FREE and the reuse of a number; the run-time errors; KEY in a console.
DIM SHARED n AS LONG, a AS LONG, b AS LONG
DIM t AS LONG, u AS LONG, v AS LONG, x AS LONG
ON ERROR GOTO oops
ON TIMER(0.6) GOSUB tick
_DELAY 0.9
PRINT "1. before TIMER ON:"; n
TIMER ON
_DELAY 1.5
TIMER OFF
PRINT "2. 1.5 s at 0.6 s:"; n
n = 0
TIMER ON
TIMER STOP
_DELAY 0.9
PRINT "3. stopped for 0.9 s:"; n
TIMER ON
PRINT "4. right after TIMER ON:"; n
TIMER OFF
n = 0
_DELAY 0.9
PRINT "5. off for 0.9 s:"; n
x = 3
ON TIMER(0.6) show x * 2
x = 100
GOSUB once
ON TIMER(0.6) small 300
GOSUB once
ON TIMER(0.6) wide 2.5
GOSUB once
ON TIMER(0.6) u16 -5
GOSUB once
setup
GOSUB once
t = _FREETIMER
u = _FREETIMER
PRINT "6. two timers:"; t; u
ON TIMER(t, 0.6) GOSUB ta
ON TIMER(u, 1) GOSUB tb
TIMER(t) ON
TIMER(u) ON
_DELAY 1.5
TIMER(t) OFF
TIMER(u) OFF
PRINT "7. a, b after 1.5 s:"; a; b
TIMER(t) FREE
v = _FREETIMER
PRINT "8. _FREETIMER after FREE:"; v
PRINT "9. ON TIMER(9, 1):";: ON TIMER(9, 1) GOSUB ta: PRINT " done"
PRINT "10. TIMER FREE:";: TIMER FREE: PRINT " done"
PRINT "11. TIMER(u + 0.6) ON:";: TIMER(u + 0.6) ON: PRINT " done"
PRINT "12. TIMER(u + 0.4) ON:";: TIMER(u + 0.4) ON: PRINT " done"
TIMER(u) OFF
PRINT "13. ON KEY(0.7):";: ON KEY(0.7) GOSUB h: PRINT " done"
PRINT "14. ON KEY(31.6):";: ON KEY(31.6) GOSUB h: PRINT " done"
PRINT "15. KEY(31.6) ON:";: KEY(31.6) ON: PRINT " done"
PRINT "16. KEY 40, x:";: KEY 40, "x": PRINT " done"
KEY(0) ON
KEY(0) OFF
KEY ON
KEY OFF
KEY 1, "abc"
KEY 15, CHR$(0) + CHR$(30)
PRINT "17. KEY LIST:"
KEY LIST
SYSTEM

once:
TIMER ON
_DELAY 0.9
TIMER OFF
RETURN

tick:
n = n + 1
RETURN

ta:
a = a + 1
RETURN

tb:
b = b + 1
RETURN

h:
PRINT "main label h"
RETURN

oops:
PRINT " error"; ERR;
RESUME NEXT

SUB show (k AS INTEGER)
    PRINT "show"; k
    k = k + 1
END SUB

SUB small (k AS _BYTE)
    PRINT "small"; k
END SUB

SUB wide (k AS _INTEGER64)
    PRINT "wide"; k
END SUB

SUB u16 (k AS _UNSIGNED INTEGER)
    PRINT "u16"; k
END SUB

SUB setup
    ON TIMER(0.6) GOSUB h
END SUB
