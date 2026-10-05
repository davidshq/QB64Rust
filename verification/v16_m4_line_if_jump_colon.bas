$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4, group 6 review): does a statement after THEN 10 or GOTO 20 and a colon belong to the branch?
IF 0 THEN 10: PRINT "after THEN 10 (outside the IF)"
PRINT "a"
10 PRINT "b"
IF 0 GOTO 20: PRINT "after GOTO 20 (outside the IF)"
20 PRINT "c"
IF 1 THEN PRINT "d" ELSE 30: PRINT "after ELSE 30 (outside the IF)"
30 PRINT "e"
SYSTEM
