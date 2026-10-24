$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RUN of a line number from a SUB.
PRINT "top"
100 PRINT "at 100"
DIM SHARED a AS LONG
c = 0
IF _FILEEXISTS("v57_r05.tmp") THEN OPEN "v57_r05.tmp" FOR INPUT AS #1: INPUT #1, c: CLOSE #1
c = c + 1
OPEN "v57_r05.tmp" FOR OUTPUT AS #1: PRINT #1, c: CLOSE #1
PRINT "run"; c; "a ="; a
a = 5
IF c < 2 THEN s
KILL "v57_r05.tmp"
SYSTEM
SUB s
    RUN 100
END SUB
