$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (13_if_goto)
c = 1
IF c GOTO 100
PRINT "skipped"
100 PRINT "at 100"
IF c = 0 GOTO 200 ELSE 300
200 PRINT "at 200 (wrong)"
300 PRINT "at 300"
IF c THEN 400 ELSE PRINT "else"
400 PRINT "at 400"
SYSTEM
