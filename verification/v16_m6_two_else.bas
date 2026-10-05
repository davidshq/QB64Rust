$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): two $ELSE in one $IF.
$IF WIN THEN
PRINT "a"
$ELSE
PRINT "b"
$ELSE
PRINT "c"
$END IF
SYSTEM
