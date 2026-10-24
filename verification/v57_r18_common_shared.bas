$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RUN clears COMMON SHARED variables too.
COMMON SHARED k AS LONG
c = 0
IF _FILEEXISTS("v57_r18.tmp") THEN OPEN "v57_r18.tmp" FOR INPUT AS #1: INPUT #1, c: CLOSE #1
c = c + 1
OPEN "v57_r18.tmp" FOR OUTPUT AS #1: PRINT #1, c: CLOSE #1
PRINT "run"; c; "k ="; k
k = 3
IF c < 2 THEN RUN
KILL "v57_r18.tmp"
SYSTEM
