$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RUN with no argument from a FUNCTION.
PRINT "top"
c = 0
IF _FILEEXISTS("v57_r07.tmp") THEN OPEN "v57_r07.tmp" FOR INPUT AS #1: INPUT #1, c: CLOSE #1
c = c + 1
OPEN "v57_r07.tmp" FOR OUTPUT AS #1: PRINT #1, c: CLOSE #1
PRINT "run"; c
IF c < 2 THEN x = f
KILL "v57_r07.tmp"
SYSTEM
FUNCTION f
    RUN
END FUNCTION
