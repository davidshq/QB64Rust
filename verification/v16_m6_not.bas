$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): NOT in a $IF condition.
$IF NOT WIN THEN
PRINT "NOT WIN: true"
$ELSE
PRINT "NOT WIN: false"
$END IF
SYSTEM
