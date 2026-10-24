$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RUN while ON TIMER is on: is the timer still on after?
c = 0
IF _FILEEXISTS("v57_r11.tmp") THEN OPEN "v57_r11.tmp" FOR INPUT AS #1: INPUT #1, c: CLOSE #1
c = c + 1
OPEN "v57_r11.tmp" FOR OUTPUT AS #1: PRINT #1, c: CLOSE #1
ON TIMER(0.05) GOSUB t
TIMER ON
_DELAY 0.3
PRINT "run"; c; "ticks"; (n > 0)
IF c < 2 THEN RUN
KILL "v57_r11.tmp"
SYSTEM
t:
n = n + 1
RETURN
