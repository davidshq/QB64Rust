$CONSOLE:ONLY
n = 0
again:
n = n + 1
IF n < 3 THEN GOTO again
PRINT "n ="; n
GOTO 100
PRINT "This line is skipped."
100 PRINT "Line 100 is a line number."
END
