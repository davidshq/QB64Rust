$CONSOLE:ONLY
' Verification (m4-handlers-logging-checking, task 1.2): when an error raised below $CHECKING:OFF is serviced
' (statements, a block, a label, a SUB defined below OFF and called from checked code, an included file that
' turns checking off), RESUME and RESUME NEXT after it, and an ON TIMER that comes due below OFF.
ON ERROR GOTO h
PRINT "1. start"
$CHECKING:OFF
ERROR 5
PRINT "1a unchecked after the error"
$CHECKING:ON
PRINT "1b checked"
PRINT "1c"
n = 0
$CHECKING:OFF
n = n + 1: IF n < 3 THEN ERROR 6
PRINT "2a n ="; n
$CHECKING:ON
PRINT "2b n ="; n
$CHECKING:OFF
FOR i = 1 TO 2
    IF i = 1 THEN ERROR 7
    PRINT "3a i ="; i
NEXT
lbl:
PRINT "3b at the label"
$CHECKING:ON
PRINT "3c"
s
PRINT "4b after s"
v = f
PRINT "5b after f"; v
'$INCLUDE: 'v50_inc/off.bi'
PRINT "6b after the include"
ERROR 9
PRINT "6c"
$CHECKING:ON
PRINT "6d checked"
ticks = 0
ON TIMER(1) GOSUB tick
TIMER ON
t0# = TIMER(.001)
$CHECKING:OFF
DO
LOOP UNTIL TIMER(.001) - t0# > 1.5 OR TIMER(.001) < t0#
PRINT "7a loop done, ticks ="; ticks
$CHECKING:ON
PRINT "7b ticks ="; ticks
TIMER OFF
PRINT "8. end"
SYSTEM

h:
PRINT "handler"; ERR
IF ERR = 6 THEN RESUME
RESUME NEXT

tick:
ticks = ticks + 1
PRINT "tick"
TIMER OFF
RETURN

$CHECKING:OFF
SUB s
    ERROR 8
    PRINT "4a in s"
END SUB

FUNCTION f
    ERROR 10
    PRINT "5a in f"
    f = 3
END FUNCTION
$CHECKING:ON
