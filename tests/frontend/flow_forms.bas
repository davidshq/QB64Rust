' TEST: parse-ok
$CONSOLE:ONLY
' Control transfer (m2-parser-breadth task 7.3, design D5): every form parses; sema marks them "not supported
' yet". The old compiler accepts this file (`qb64pe.exe -z`, 2026-10-07).
n = 2
ON n GOTO a, b
ON n + 1 GOSUB a, , 30
ON TIMER(1) GOSUB a
t = _FREETIMER
ON TIMER(t, 0.5) handler
ON KEY(1) GOSUB b
ON STRIG(0) GOSUB a
TIMER ON
TIMER(t) OFF
KEY(1) STOP
TIMER(t) FREE
ON ERROR GOTO _NEWHANDLER h
a:
b:
30 STOP
h:
IF n = 7 THEN RUN
IF n = 8 THEN RUN 30
IF n = 9 THEN RUN "other.bas"
IF n = 10 THEN END 3
SYSTEM 1

SUB handler
END SUB
