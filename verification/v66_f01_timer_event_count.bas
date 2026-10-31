$CONSOLE:ONLY
' Verification (SOMEDAY.md, behaviours to review, ON TIMER's count of events (libqb adds 1 twice per event, QBMAIN takes 1 away once, study/archive/02 9.2)): events that come during TIMER STOP and during a slow handler: how many handler runs follow.
DIM SHARED n AS LONG, slow AS LONG
ON TIMER(0.05) GOSUB tick
TIMER ON
t# = TIMER(0.001)
DO: LOOP UNTIL n >= 2 OR TIMER(0.001) - t# > 5
TIMER STOP
c = n
w# = TIMER(0.001): DO: LOOP UNTIL TIMER(0.001) - w# > 0.3
PRINT "1. runs while stopped:"; n - c
TIMER ON
d = n
e = n
f = n
TIMER OFF
PRINT "2. runs at TIMER ON:"; d - c; e - c; f - c
n = 0: slow = 1
TIMER ON
t# = TIMER(0.001)
DO: LOOP UNTIL n >= 1 OR TIMER(0.001) - t# > 5
a = n
b = n
g = n
TIMER OFF
PRINT "3. runs after a slow handler:"; a; b; g
SYSTEM
tick:
n = n + 1
IF slow AND n = 1 THEN
    w# = TIMER(0.001): DO: LOOP UNTIL TIMER(0.001) - w# > 0.3
END IF
RETURN
