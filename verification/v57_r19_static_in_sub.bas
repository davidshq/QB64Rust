$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RUN clears a STATIC variable of a SUB.
c = 0
IF _FILEEXISTS("v57_r19.tmp") THEN OPEN "v57_r19.tmp" FOR INPUT AS #1: INPUT #1, c: CLOSE #1
c = c + 1
OPEN "v57_r19.tmp" FOR OUTPUT AS #1: PRINT #1, c: CLOSE #1
s
IF c < 2 THEN RUN
KILL "v57_r19.tmp"
SYSTEM
SUB s
    STATIC k AS LONG
    PRINT "k ="; k
    k = k + 1
END SUB
