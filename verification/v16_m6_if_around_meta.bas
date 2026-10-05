$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6, review): an IF opened before a $IF and closed inside it.
x = 1
IF x = 1 THEN
$IF WIN THEN
    PRINT "inside"
END IF
$END IF
SYSTEM
