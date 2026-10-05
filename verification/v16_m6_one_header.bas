$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): one IF header inside $IF, its END IF outside.
x = 1
$IF WIN THEN
IF x = 1 THEN
$END IF
    PRINT "inside"
END IF
SYSTEM
