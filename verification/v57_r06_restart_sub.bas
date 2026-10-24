$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RUN with no argument from a SUB.
PRINT "top"
DIM SHARED a AS LONG
c = 0
IF _FILEEXISTS("v57_r06.tmp") THEN OPEN "v57_r06.tmp" FOR INPUT AS #1: INPUT #1, c: CLOSE #1
c = c + 1
OPEN "v57_r06.tmp" FOR OUTPUT AS #1: PRINT #1, c: CLOSE #1
PRINT "run"; c; "a ="; a
a = 5
IF c < 2 THEN s
KILL "v57_r06.tmp"
SYSTEM
SUB s
    RUN
END SUB
