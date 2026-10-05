$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4, group 6): a whole FOR or DO block inside a single-line IF; THEN followed by a colon; THEN REM; empty branches.
c = 1
IF c = 1 THEN : FOR p = 1 TO 3: PRINT p;: NEXT p: PRINT "done"
IF c = 0 THEN DO: PRINT "no": LOOP ELSE PRINT "else"
IF c = 1 THEN REM a comment makes this a single-line IF
PRINT "after rem"
IF c = 1 THEN PRINT "empty else" ELSE
IF c = 0 THEN ELSE PRINT "empty then"
IF c = 1 THEN PRINT "a" ELSE IF c = 2 THEN PRINT "b" ELSE PRINT "c"
SYSTEM
