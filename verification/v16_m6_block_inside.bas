$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): $IF/$ELSE inside an IF block (not splitting it).
x = 1
IF x = 1 THEN
$IF WIN THEN
    PRINT "win"
$ELSE
    PRINT "other"
$END IF
END IF
SYSTEM
