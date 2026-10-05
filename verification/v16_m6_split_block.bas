$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): $IF splitting an IF block (two headers, one END IF).
x = 1
$IF WIN THEN
IF x = 1 THEN
$ELSE
IF x = 2 THEN
$END IF
    PRINT "inside"
END IF
SYSTEM
