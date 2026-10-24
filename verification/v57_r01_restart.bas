$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RUN with no argument restarts with the variables cleared, RND reseeded.
DIM a AS LONG, s AS STRING, x(3) AS INTEGER
c = 0
IF _FILEEXISTS("v57_r01.tmp") THEN OPEN "v57_r01.tmp" FOR INPUT AS #1: INPUT #1, c: CLOSE #1
c = c + 1
OPEN "v57_r01.tmp" FOR OUTPUT AS #1: PRINT #1, c: CLOSE #1
PRINT "run"; c; "a ="; a; "s = ["; s; "] x(2) ="; x(2); "rnd"; INT(RND * 1000)
a = 7: s = "set": x(2) = 9
OPEN "v57_r01_open.tmp" FOR OUTPUT AS #2
IF c < 3 THEN RUN
PRINT "free file"; FREEFILE
CLOSE
KILL "v57_r01.tmp": KILL "v57_r01_open.tmp"
SYSTEM
